import { SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { getMusicService } from "../../services/music/index.js";

export const leaveCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("leave")
    .setDescription("Leave the voice channel"),

  async execute({ interaction, language }) {
    getMusicService().leave(interaction.guildId!);
    await interaction.reply(t(language, "music.leave.success"));
  },

  async executePrefix({ message, language }) {
    getMusicService().leave(message.guildId!);
    await message.reply(t(language, "music.leave.success"));
  }
};
