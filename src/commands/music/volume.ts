import { SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { guildSettingsService } from "../../services/guildSettings/guildSettingsService.js";
import { getMusicService } from "../../services/music/index.js";

export const volumeCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("volume")
    .setDescription("Set the music volume")
    .addIntegerOption((option) =>
      option
        .setName("value")
        .setDescription("Volume from 1 to 100")
        .setRequired(true)
        .setMinValue(1)
        .setMaxValue(100)
    ),

  async execute({ interaction, language }) {
    const volume = interaction.options.getInteger("value", true);

    await guildSettingsService.update(interaction.guildId!, {
      musicVolume: volume
    });
    getMusicService().setVolume(interaction.guildId!, volume);

    await interaction.reply(t(language, "music.volume.changed", { volume }));
  },

  async executePrefix({ message, language, prefix, args }) {
    const volume = Number(args[0]);

    if (!Number.isInteger(volume) || volume < 1 || volume > 100) {
      await message.reply(`Usage: \`${prefix}volume <1-100>\``);
      return;
    }

    await guildSettingsService.update(message.guildId!, {
      musicVolume: volume
    });
    getMusicService().setVolume(message.guildId!, volume);

    await message.reply(t(language, "music.volume.changed", { volume }));
  }
};
