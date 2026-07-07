import { ActionRowBuilder, ButtonBuilder, ButtonStyle, EmbedBuilder } from 'discord.js';
import type { Queue, Song } from 'distube';
import { t } from '../../i18n/t.js';
import { formatDuration } from '../../utils/duration.js';

export const NOW_PLAYING_PREVIOUS_BUTTON_ID = 'nowplaying:previous';
export const NOW_PLAYING_PLAY_PAUSE_BUTTON_ID = 'nowplaying:play-pause';
export const NOW_PLAYING_NEXT_BUTTON_ID = 'nowplaying:next';
export const NOW_PLAYING_STOP_BUTTON_ID = 'nowplaying:stop';
export const NOW_PLAYING_REFRESH_BUTTON_ID = 'nowplaying:refresh';

const BAR_SIZE = 22;

function clamp(value: number, min: number, max: number) {
  return Math.min(max, Math.max(min, value));
}

function truncate(value: string, maxLength: number) {
  return value.length > maxLength ? `${value.slice(0, maxLength - 1)}…` : value;
}

function buildProgressBar(currentSeconds: number, totalSeconds: number) {
  if (!Number.isFinite(totalSeconds) || totalSeconds <= 0) {
    return '🔴 LIVE';
  }

  const progress = clamp(currentSeconds / totalSeconds, 0, 1);
  const markerIndex = clamp(Math.floor(progress * BAR_SIZE), 0, BAR_SIZE - 1);

  return `${'─'.repeat(markerIndex)}🔘${'─'.repeat(BAR_SIZE - markerIndex - 1)}`;
}

function getYouTubeVideoId(url: string | undefined) {
  if (!url) return undefined;

  try {
    const parsed = new URL(url);

    if (parsed.hostname === 'youtu.be') {
      return parsed.pathname.split('/').filter(Boolean)[0];
    }

    if (parsed.hostname.includes('youtube.com')) {
      return parsed.searchParams.get('v') ?? undefined;
    }
  } catch {
    return undefined;
  }

  return undefined;
}

function getSongImage(song: Song | undefined) {
  if (!song) return undefined;

  const streamSong = song.stream.playFromSource ? undefined : song.stream.song;
  const thumbnail = song.thumbnail ?? streamSong?.thumbnail;
  if (thumbnail) return thumbnail;

  const videoId = getYouTubeVideoId(song.url ?? streamSong?.url);
  return videoId ? `https://img.youtube.com/vi/${videoId}/hqdefault.jpg` : undefined;
}

export function buildNowPlayingEmbed(queue: Queue, language: string, song = queue.songs[0]) {
  const title = song?.name ?? t(language, 'music.nowplaying.unknownTitle');
  const totalSeconds = song?.duration ?? 0;
  const currentSeconds =
    totalSeconds > 0
      ? clamp(queue.currentTime, 0, Math.max(totalSeconds, 0))
      : Math.max(queue.currentTime, 0);
  const progressBar = buildProgressBar(currentSeconds, totalSeconds);
  const timeLabel =
    totalSeconds > 0
      ? `${formatDuration(currentSeconds)} / ${formatDuration(totalSeconds)}`
      : formatDuration(currentSeconds);

  const loopLabel = queue.repeatMode === 0 ? 'Off' : queue.repeatMode === 1 ? 'Once' : 'On';

  const embed = new EmbedBuilder()
    .setColor(0xff7ac8)
    .setTitle(`🎶 ${truncate(title, 253)}`)
    .setDescription(`${progressBar}  \`${timeLabel}\``)
    .addFields(
      {
        name: 'Volume',
        value: `\`${queue.volume}%\``,
        inline: true,
      },
      {
        name: 'Loop',
        value: `\`${loopLabel}\``,
        inline: true,
      },
    );

  const image = getSongImage(song);

  if (song?.url) embed.setURL(song.url);
  if (image) embed.setThumbnail(image);

  return embed;
}

export function buildNowPlayingComponents(queue: Queue | undefined, forceDisabled = false) {
  const disabled = forceDisabled || !queue || queue.stopped;
  const playPauseEmoji = queue?.paused ? '▶️' : '⏸️';
  const playPauseLabel = queue?.paused ? 'Play' : 'Pause';

  return [
    new ActionRowBuilder<ButtonBuilder>().addComponents(
      new ButtonBuilder()
        .setCustomId(NOW_PLAYING_PREVIOUS_BUTTON_ID)
        .setEmoji('⏮️')
        .setLabel('Prev')
        .setStyle(ButtonStyle.Secondary)
        .setDisabled(disabled),
      new ButtonBuilder()
        .setCustomId(NOW_PLAYING_PLAY_PAUSE_BUTTON_ID)
        .setEmoji(playPauseEmoji)
        .setLabel(playPauseLabel)
        .setStyle(ButtonStyle.Primary)
        .setDisabled(disabled),
      new ButtonBuilder()
        .setCustomId(NOW_PLAYING_NEXT_BUTTON_ID)
        .setEmoji('⏭️')
        .setLabel('Next')
        .setStyle(ButtonStyle.Secondary)
        .setDisabled(disabled),
      new ButtonBuilder()
        .setCustomId(NOW_PLAYING_STOP_BUTTON_ID)
        .setEmoji('⏹️')
        .setLabel('Stop')
        .setStyle(ButtonStyle.Danger)
        .setDisabled(disabled),
      new ButtonBuilder()
        .setCustomId(NOW_PLAYING_REFRESH_BUTTON_ID)
        .setEmoji('🔄')
        .setLabel('Refresh')
        .setStyle(ButtonStyle.Secondary)
        .setDisabled(disabled),
    ),
  ];
}
