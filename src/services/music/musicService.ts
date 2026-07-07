import {
  GuildMember,
  type ChatInputCommandInteraction,
  type GuildTextBasedChannel,
  type Message,
  type VoiceBasedChannel,
} from 'discord.js';
import { RepeatMode, Song, type DisTube, type Queue } from 'distube';
import { logger } from '../../logging/logger.js';
import { BotError } from '../commandUX/errors.js';
import { assertBotCanUseVoice, assertMemberInVoice } from '../permissions/checks.js';

const SPOTIFY_SHARE_HOSTS = new Set(['spotify.link', 'spotify.app.link']);
const SPOTIFY_EMBED_TIMEOUT_MS = 10_000;

type SpotifyTrack = {
  title?: string;
  name?: string;
  subtitle?: string;
  artists?: Array<{ name?: string }>;
};

type SpotifyList = {
  tracks?: SpotifyTrack[];
};

type SpotifyPluginWithApi = {
  api?: {
    getData(url: string): Promise<SpotifyTrack | SpotifyList>;
  };
};

type SpotifyResult =
  | { kind: 'track'; query: string }
  | { kind: 'playlist'; tracks: Array<{ query: string; title: string }> };

export type MusicPlayProgress = {
  queued: number;
  total: number;
  failed: number;
  done: boolean;
  title?: string;
};

export type MusicPlayResult =
  { kind: 'song' } | { kind: 'playlist'; queued: number; total: number; failed: number };

type ProgressHandler = (progress: MusicPlayProgress) => unknown | Promise<unknown>;

function isSpotifyShortUrl(input: string) {
  try {
    const url = new URL(input.trim());
    const host = url.hostname.toLowerCase();

    return SPOTIFY_SHARE_HOSTS.has(host) || host.endsWith('.spotify.link');
  } catch {
    return false;
  }
}

function getSpotifyOpenPath(input: string) {
  try {
    const url = new URL(input.trim());
    if (url.hostname.toLowerCase() !== 'open.spotify.com') return;

    const parts = url.pathname.split('/').filter(Boolean);
    const typeIndex = parts.findIndex((part) => ['track', 'playlist', 'album'].includes(part));
    const type = parts[typeIndex];
    const id = parts[typeIndex + 1];

    return type && id ? { type, id } : undefined;
  } catch {
    return;
  }
}

function spotifyTrackTitle(track: SpotifyTrack) {
  return track.title ?? track.name ?? 'Unknown track';
}

function spotifyTrackQuery(track: SpotifyTrack) {
  const title = spotifyTrackTitle(track);
  const artist =
    track.artists
      ?.map((artist) => artist.name)
      .filter(Boolean)
      .join(', ') ?? track.subtitle;

  return [title, artist].filter(Boolean).join(' ').trim();
}

async function fetchTextWithTimeout(url: string) {
  const controller = new AbortController();
  const timeout = setTimeout(() => controller.abort(), SPOTIFY_EMBED_TIMEOUT_MS);

  try {
    const response = await fetch(url, {
      signal: controller.signal,
      headers: {
        'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
      },
    });

    return response.ok ? await response.text() : undefined;
  } finally {
    clearTimeout(timeout);
  }
}

function spotifyDataToResult(data: SpotifyTrack | SpotifyList, type?: string) {
  if (type === 'track' || !Array.isArray((data as SpotifyList).tracks)) {
    const query = spotifyTrackQuery(data as SpotifyTrack);
    return query ? ({ kind: 'track', query } as const) : undefined;
  }

  const tracks = (data as SpotifyList).tracks
    ?.map((track) => ({
      query: spotifyTrackQuery(track),
      title: spotifyTrackTitle(track),
    }))
    .filter((track) => track.query);

  return tracks?.length ? ({ kind: 'playlist', tracks } as const) : undefined;
}

async function resolveSpotifyEmbed(input: string): Promise<SpotifyResult | undefined> {
  const spotifyPath = getSpotifyOpenPath(input);
  if (!spotifyPath) return;

  const html = await fetchTextWithTimeout(
    `https://open.spotify.com/embed/${spotifyPath.type}/${spotifyPath.id}`,
  ).catch(() => undefined);
  const json = html?.match(
    /<script id="__NEXT_DATA__" type="application\/json">(.*?)<\/script>/s,
  )?.[1];
  if (!json) return;

  const entity = JSON.parse(json)?.props?.pageProps?.state?.data?.entity;
  if (!entity) return;

  if (spotifyPath.type === 'track') {
    const query = spotifyTrackQuery(entity);
    return query ? { kind: 'track', query } : undefined;
  }

  const trackList = Array.isArray(entity.trackList)
    ? (entity.trackList as SpotifyTrack[])
    : undefined;
  const tracks = trackList
    ?.map((track) => ({
      query: spotifyTrackQuery(track),
      title: spotifyTrackTitle(track),
    }))
    .filter((track) => track.query);

  return tracks?.length ? { kind: 'playlist', tracks } : undefined;
}

async function normalizeSpotifyUrl(input: string) {
  if (!isSpotifyShortUrl(input)) return input;

  for (const method of ['HEAD', 'GET'] as const) {
    try {
      const response = await fetch(input, {
        method,
        redirect: 'follow',
        headers: {
          'user-agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36',
        },
      });
      const resolvedUrl = response.url;
      await response.body?.cancel().catch(() => undefined);

      if (resolvedUrl.includes('open.spotify.com/')) {
        return resolvedUrl;
      }
    } catch {
      // Try the next method, then fall back to the original input.
    }
  }

  return input;
}

export type LoopMode = 'off' | 'on' | 'once';

export class MusicService {
  constructor(private distube: DisTube) {}

  async play(options: {
    interaction: ChatInputCommandInteraction;
    query: string;
    volume: number;
    onProgress?: ProgressHandler;
  }) {
    const member = options.interaction.member as GuildMember;

    return this.playForMember({
      guildId: options.interaction.guildId!,
      member,
      me: options.interaction.guild?.members.me,
      query: options.query,
      volume: options.volume,
      textChannel: (options.interaction.channel ?? undefined) as GuildTextBasedChannel | undefined,
      metadata: {
        interactionId: options.interaction.id,
      },
      onProgress: options.onProgress,
    });
  }

  async playFromMessage(options: {
    message: Message;
    query: string;
    volume: number;
    onProgress?: ProgressHandler;
  }) {
    const member = options.message.member as GuildMember | null;
    if (!member) throw new BotError('GUILD_ONLY');

    return this.playForMember({
      guildId: options.message.guildId!,
      member,
      me: options.message.guild?.members.me,
      query: options.query,
      volume: options.volume,
      textChannel: options.message.channel as GuildTextBasedChannel,
      metadata: {
        messageId: options.message.id,
      },
      onProgress: options.onProgress,
    });
  }

  private async playForMember(options: {
    guildId: string;
    member: GuildMember;
    me: GuildMember | null | undefined;
    query: string;
    volume: number;
    textChannel: GuildTextBasedChannel | undefined;
    metadata: Record<string, string>;
    onProgress?: ProgressHandler;
  }): Promise<MusicPlayResult> {
    const voiceChannel = assertMemberInVoice(options.member);

    if (options.me) {
      assertBotCanUseVoice(voiceChannel, options.me);
    }

    const query = await normalizeSpotifyUrl(options.query);
    const spotify = await this.resolveSpotifyInput(query);
    const playOptions = {
      member: options.member,
      textChannel: options.textChannel,
      metadata: options.metadata,
    };

    if (spotify?.kind === 'playlist') {
      const [firstTrack, ...remainingTracks] = spotify.tracks;
      if (!firstTrack) throw new Error('Spotify playlist did not contain playable tracks');

      await this.distube.play(voiceChannel as VoiceBasedChannel, firstTrack.query, playOptions);
      this.applyQueueVolume(options.guildId, options.volume);

      const total = spotify.tracks.length;
      let queued = 1;
      let failed = 0;
      await options.onProgress?.({
        queued,
        total,
        failed,
        done: false,
        title: firstTrack.title,
      });

      const remaining = await this.queueSpotifyTracks({
        guildId: options.guildId,
        tracks: remainingTracks,
        member: options.member,
        metadata: options.metadata,
        total,
        initialQueued: queued,
        initialFailed: failed,
        onProgress: options.onProgress,
      });

      queued += remaining.queued;
      failed += remaining.failed;
      await options.onProgress?.({ queued, total, failed, done: true });

      return { kind: 'playlist', queued, total, failed };
    }

    await this.distube.play(
      voiceChannel as VoiceBasedChannel,
      spotify?.kind === 'track' ? spotify.query : query,
      playOptions,
    );

    this.applyQueueVolume(options.guildId, options.volume);
    return { kind: 'song' };
  }

  private async resolveSpotifyInput(input: string): Promise<SpotifyResult | undefined> {
    const spotifyPath = getSpotifyOpenPath(input);
    if (!spotifyPath) return;

    const spotifyPlugin = this.distube.plugins.find(
      (plugin) => plugin.constructor.name === 'SpotifyPlugin',
    ) as SpotifyPluginWithApi | undefined;
    const data = await spotifyPlugin?.api?.getData(input).catch((error: unknown) => {
      logger.warn({ err: error }, 'Failed to resolve Spotify through API');
      return undefined;
    });

    return data ? spotifyDataToResult(data, spotifyPath.type) : resolveSpotifyEmbed(input);
  }

  private applyQueueVolume(guildId: string, volume: number) {
    const queue = this.getQueue(guildId);
    queue?.setVolume(volume);
  }

  private async queueSpotifyTracks(options: {
    guildId: string;
    tracks: Array<{ query: string; title: string }>;
    member: GuildMember;
    metadata: Record<string, string>;
    total: number;
    initialQueued: number;
    initialFailed: number;
    onProgress?: ProgressHandler;
  }) {
    let queued = 0;
    let failed = 0;

    for (const [index, track] of options.tracks.entries()) {
      const queue = this.getQueue(options.guildId);
      if (!queue) {
        failed += options.tracks.length - index;
        break;
      }

      try {
        const resolved = await this.distube.handler.resolve(track.query, {
          member: options.member,
          metadata: options.metadata,
        });

        if (resolved instanceof Song) {
          queue.addToQueue(resolved);
          queued += 1;
        } else {
          failed += 1;
        }
      } catch (error) {
        failed += 1;
        logger.warn(
          { err: error, guildId: options.guildId, query: track.query },
          'Failed to queue Spotify playlist track',
        );
      }

      await options.onProgress?.({
        queued: options.initialQueued + queued,
        total: options.total,
        failed: options.initialFailed + failed,
        done: false,
        title: track.title,
      });
    }

    return { queued, failed };
  }

  getQueue(guildId: string): Queue | undefined {
    return this.distube.getQueue(guildId);
  }

  async skip(guildId: string) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError('NO_QUEUE');

    if (queue.songs.length <= 1) {
      queue.stop();
      return;
    }

    return queue.skip();
  }

  async previous(guildId: string) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError('NO_QUEUE');

    return queue.previous();
  }

  async togglePause(guildId: string) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError('NO_QUEUE');

    if (queue.paused) {
      await queue.resume();
      return false;
    }

    await queue.pause();
    return true;
  }

  setVolume(guildId: string, volume: number) {
    const queue = this.getQueue(guildId);
    queue?.setVolume(volume);
  }

  setLoop(guildId: string, mode?: LoopMode) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError('NO_QUEUE');

    const nextMode: LoopMode = mode ?? (queue.repeatMode === RepeatMode.QUEUE ? 'off' : 'on');

    const repeatMode =
      nextMode === 'off'
        ? RepeatMode.DISABLED
        : nextMode === 'once'
          ? RepeatMode.SONG
          : RepeatMode.QUEUE;

    queue.setRepeatMode(repeatMode);
    return nextMode;
  }

  stop(guildId: string) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError('NO_QUEUE');

    queue.stop();
  }

  seek(guildId: string, seconds: number) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError('NO_QUEUE');

    queue.seek(seconds);
  }

  leave(guildId: string) {
    const queue = this.getQueue(guildId);

    if (queue) {
      queue.voice.leave();
      return;
    }

    const voice = this.distube.voices.get(guildId);
    if (voice) {
      voice.leave();
      return;
    }

    throw new BotError('NOT_CONNECTED');
  }

  move(guildId: string, from: number, to: number) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError('NO_QUEUE');

    const songs = queue.songs;
    const fromIndex = from - 1;
    const toIndex = to - 1;

    if (fromIndex <= 0 || toIndex <= 0 || fromIndex >= songs.length || toIndex >= songs.length) {
      throw new BotError('INVALID_POSITION');
    }

    const [item] = songs.splice(fromIndex, 1);
    if (!item) throw new BotError('INVALID_POSITION');

    songs.splice(toIndex, 0, item);
  }
}
