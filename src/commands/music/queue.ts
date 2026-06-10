import { SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { sendPaginatedMessage } from "../../services/commandUX/pagination.js";
import { getMusicService } from "../../services/music/index.js";
import { buildQueuePage, getQueuePageCount } from "../../services/music/queueView.js";

export const queueCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("queue")
    .setDescription("Show the current music queue"),

  async execute({ interaction, language }) {
    const queue = getMusicService().getQueue(interaction.guildId!);

    if (!queue || queue.songs.length === 0) {
      await interaction.reply(t(language, "music.queue.empty"));
      return;
    }

    await sendPaginatedMessage({
      ownerId: interaction.user.id,
      language,
      getTotalPages: () => {
        const currentQueue = getMusicService().getQueue(interaction.guildId!);
        return currentQueue ? getQueuePageCount(currentQueue) : 1;
      },
      buildPage: ({ page, totalPages }) => {
        const currentQueue = getMusicService().getQueue(interaction.guildId!);
        return currentQueue && currentQueue.songs.length > 0
          ? buildQueuePage(currentQueue, language, page, totalPages)
          : { content: t(language, "music.queue.empty"), embeds: [] };
      },
      send: async (payload) => {
        await interaction.reply(payload);
        return interaction.fetchReply();
      }
    });
  },

  async executePrefix({ message, language }) {
    const queue = getMusicService().getQueue(message.guildId!);

    if (!queue || queue.songs.length === 0) {
      await message.reply(t(language, "music.queue.empty"));
      return;
    }

    await sendPaginatedMessage({
      ownerId: message.author.id,
      language,
      getTotalPages: () => {
        const currentQueue = getMusicService().getQueue(message.guildId!);
        return currentQueue ? getQueuePageCount(currentQueue) : 1;
      },
      buildPage: ({ page, totalPages }) => {
        const currentQueue = getMusicService().getQueue(message.guildId!);
        return currentQueue && currentQueue.songs.length > 0
          ? buildQueuePage(currentQueue, language, page, totalPages)
          : { content: t(language, "music.queue.empty"), embeds: [] };
      },
      send: (payload) => message.reply(payload)
    });
  }
};
