import { SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { getMusicService } from "../../services/music/index.js";

export const skipCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("skip")
    .setDescription("Skip the current song"),

  async execute({ interaction, language }) {
    await getMusicService().skip(interaction.guildId!);
    await interaction.reply(t(language, "music.skip.success"));
  },

  async executePrefix({ message, language }) {
    await getMusicService().skip(message.guildId!);
    await message.reply(t(language, "music.skip.success"));
  }
};
