import { SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { getMusicService } from "../../services/music/index.js";

export const stopCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("stop")
    .setDescription("Stop playback and clear the queue"),

  async execute({ interaction, language }) {
    getMusicService().stop(interaction.guildId!);
    await interaction.reply(t(language, "music.stop.success"));
  },

  async executePrefix({ message, language }) {
    getMusicService().stop(message.guildId!);
    await message.reply(t(language, "music.stop.success"));
  }
};
