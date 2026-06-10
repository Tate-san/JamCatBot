import { SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { getMusicService } from "../../services/music/index.js";

export const moveCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("move")
    .setDescription("Move a song within the queue")
    .addIntegerOption((option) =>
      option
        .setName("from")
        .setDescription("Current queue position")
        .setMinValue(1)
        .setRequired(true)
    )
    .addIntegerOption((option) =>
      option
        .setName("to")
        .setDescription("New queue position")
        .setMinValue(1)
        .setRequired(true)
    ),

  async execute({ interaction, language }) {
    const from = interaction.options.getInteger("from", true);
    const to = interaction.options.getInteger("to", true);

    getMusicService().move(interaction.guildId!, from, to);

    await interaction.reply(t(language, "music.move.success", { from, to }));
  },

  async executePrefix({ message, language, prefix, args }) {
    const from = Number(args[0]);
    const to = Number(args[1]);

    if (!Number.isInteger(from) || !Number.isInteger(to)) {
      await message.reply(`Usage: \`${prefix}move <from> <to>\``);
      return;
    }

    getMusicService().move(message.guildId!, from, to);

    await message.reply(t(language, "music.move.success", { from, to }));
  }
};
