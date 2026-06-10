import { SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { BotError } from "../../services/commandUX/errors.js";
import { getMusicService } from "../../services/music/index.js";
import { formatDuration, parseDuration } from "../../utils/duration.js";

export const seekCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("seek")
    .setDescription("Seek to a timestamp in the current song")
    .addStringOption((option) =>
      option
        .setName("time")
        .setDescription("Timestamp, for example 75, 1:15, or 1:02:03")
        .setRequired(true)
    ),

  async execute({ interaction, language }) {
    const input = interaction.options.getString("time", true);
    const seconds = parseDuration(input);

    if (seconds === undefined) {
      throw new BotError("INVALID_TIME");
    }

    getMusicService().seek(interaction.guildId!, seconds);
    await interaction.reply(
      t(language, "music.seek.success", { time: formatDuration(seconds) })
    );
  },

  async executePrefix({ message, language, prefix, args }) {
    const input = args[0];
    const seconds = input ? parseDuration(input) : undefined;

    if (seconds === undefined) {
      if (!input) await message.reply(`Usage: \`${prefix}seek <time>\``);
      else throw new BotError("INVALID_TIME");
      return;
    }

    getMusicService().seek(message.guildId!, seconds);
    await message.reply(
      t(language, "music.seek.success", { time: formatDuration(seconds) })
    );
  }
};
