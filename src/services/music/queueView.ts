import { EmbedBuilder } from "discord.js";
import type { Queue } from "distube";
import { t } from "../../i18n/t.js";
import { formatDuration } from "../../utils/duration.js";

const PAGE_SIZE = 10;

function clampPage(page: number, totalPages: number) {
  return Math.min(Math.max(page, 0), Math.max(totalPages - 1, 0));
}

function truncate(value: string | undefined, maxLength: number) {
  const safeValue = value || "Unknown";
  return safeValue.length > maxLength
    ? `${safeValue.slice(0, maxLength - 1)}…`
    : safeValue;
}

function formatSongLine(queueIndex: number, song: Queue["songs"][number]) {
  const duration = song.isLive ? "LIVE" : song.formattedDuration;
  return `\`${queueIndex}.\` **${truncate(song.name, 70)}** — \`${duration}\``;
}

export function getQueuePageCount(queue: Queue) {
  return Math.max(Math.ceil(Math.max(queue.songs.length - 1, 0) / PAGE_SIZE), 1);
}

export function buildQueuePage(
  queue: Queue,
  language: string,
  page = 0,
  totalPages = getQueuePageCount(queue)
) {
  const currentSong = queue.songs[0];
  const upcomingSongs = queue.songs.slice(1);
  const currentPage = clampPage(page, totalPages);
  const pageStart = currentPage * PAGE_SIZE;
  const pageSongs = upcomingSongs.slice(pageStart, pageStart + PAGE_SIZE);
  const queueLines = pageSongs.map((song, index) =>
    formatSongLine(pageStart + index + 2, song)
  );

  const embed = new EmbedBuilder()
    .setColor(0x8bd5ff)
    .setTitle(`🎼 ${t(language, "music.queue.title")}`)
    .setDescription(
      currentSong
        ? t(language, "music.queue.summary", {
            count: Math.max(queue.songs.length - 1, 0)
          })
        : t(language, "music.queue.empty")
    )
    .addFields({
      name: t(language, "music.queue.nowPlaying"),
      value: currentSong
        ? `▶️ **${truncate(currentSong.name, 100)}**\n\`${formatDuration(
            queue.currentTime
          )} / ${currentSong.isLive ? "LIVE" : currentSong.formattedDuration}\``
        : t(language, "music.queue.empty"),
      inline: false
    })
    .setFooter({
      text: t(language, "music.queue.page", {
        page: currentPage + 1,
        pages: totalPages
      })
    });

  embed.addFields({
    name: t(language, "music.queue.upNext"),
    value: queueLines.length
      ? queueLines.join("\n")
      : t(language, "music.queue.noUpcoming"),
    inline: false
  });

  if (currentSong?.thumbnail) embed.setThumbnail(currentSong.thumbnail);

  return { embeds: [embed] };
}
