import { DisTubeError, ExtractorPlugin, Playlist, Song, type ResolveOptions } from 'distube';
import { spawn, type SpawnOptionsWithoutStdio } from 'node:child_process';
import { existsSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, resolve } from 'node:path';
import { env } from '../../config/env.js';

const require = createRequire(import.meta.url);
const YTDLP_TIMEOUT_MS = 45_000;

type YtDlpFlags = {
  dumpSingleJson?: boolean;
  noWarnings?: boolean;
  preferFreeFormats?: boolean;
  skipDownload?: boolean;
  simulate?: boolean;
  format?: string;
};

type YtDlpInfo = {
  id?: string | number;
  title?: string;
  fulltitle?: string;
  extractor?: string;
  webpage_url?: string;
  original_url?: string;
  url?: string;
  is_live?: boolean;
  thumbnail?: string;
  thumbnails?: Array<{ url?: string }>;
  duration?: number;
  uploader?: string;
  uploader_url?: string;
  entries?: YtDlpInfo[];
};

function getBundledYtDlpPath() {
  try {
    const entry = require.resolve('@distube/yt-dlp');
    const executable = `yt-dlp${process.platform === 'win32' ? '.exe' : ''}`;
    const bundledPath = resolve(dirname(entry), '..', 'bin', executable);

    return existsSync(bundledPath) ? bundledPath : undefined;
  } catch {
    return undefined;
  }
}

function getYtDlpPath() {
  return process.env.YTDLP_PATH || getBundledYtDlpPath() || env.YTDLP_PATH;
}

function toArgs(input: string, flags: YtDlpFlags) {
  const args = [input];

  if (flags.dumpSingleJson) args.push('--dump-single-json');
  if (flags.noWarnings) args.push('--no-warnings');
  if (flags.preferFreeFormats) args.push('--prefer-free-formats');
  if (flags.skipDownload) args.push('--skip-download');
  if (flags.simulate) args.push('--simulate');
  if (flags.format) args.push('--format', flags.format);

  return args;
}

function parseJsonOutput(output: string) {
  const trimmed = output.trim();

  try {
    return JSON.parse(trimmed) as YtDlpInfo;
  } catch {
    const start = trimmed.search(/[\[{]/);
    const end = Math.max(trimmed.lastIndexOf('}'), trimmed.lastIndexOf(']'));

    if (start === -1 || end === -1 || end <= start) {
      throw new Error(`yt-dlp did not return JSON. Output: ${trimmed}`);
    }

    return JSON.parse(trimmed.slice(start, end + 1)) as YtDlpInfo;
  }
}

function getJson(input: string, flags: YtDlpFlags, options?: SpawnOptionsWithoutStdio) {
  const child = spawn(getYtDlpPath(), toArgs(input, flags), options);

  return new Promise<YtDlpInfo>((resolvePromise, reject) => {
    let stdout = '';
    let stderr = '';
    let settled = false;
    const timeout = setTimeout(() => {
      if (settled) return;
      settled = true;
      child.kill('SIGKILL');
      reject(new Error(`yt-dlp timed out after ${YTDLP_TIMEOUT_MS / 1000}s`));
    }, YTDLP_TIMEOUT_MS);

    child.stdout?.on('data', (chunk) => {
      stdout += chunk;
    });

    child.stderr?.on('data', (chunk) => {
      stderr += chunk;
    });

    child.on('close', (code) => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timeout);

      if (code !== 0) {
        reject(new Error(stderr || stdout || `yt-dlp exited with code ${code}`));
        return;
      }

      try {
        resolvePromise(parseJsonOutput(stdout));
      } catch (error) {
        reject(error);
      }
    });

    child.on('error', (error) => {
      if (settled) {
        return;
      }
      settled = true;
      clearTimeout(timeout);
      reject(error);
    });
  });
}

function isPlaylist(info: YtDlpInfo): info is YtDlpInfo & { entries: YtDlpInfo[] } {
  return Array.isArray(info.entries);
}

function isSpotifyShortUrl(input: string) {
  try {
    const url = new URL(input);
    const host = url.hostname.toLowerCase();

    return host === 'spotify.link' || host === 'spotify.app.link' || host.endsWith('.spotify.link');
  } catch {
    return false;
  }
}

export class YtDlpExtractorPlugin extends ExtractorPlugin {
  override validate(url: string) {
    return !isSpotifyShortUrl(url);
  }

  override async resolve<T>(url: string, options: ResolveOptions<T>) {
    const info = await this.getInfo(url);

    if (isPlaylist(info)) {
      if (info.entries.length === 0) {
        throw new DisTubeError('YTDLP_ERROR', 'The playlist is empty');
      }

      return new Playlist(
        {
          source: info.extractor ?? 'yt-dlp',
          songs: info.entries.map((entry) => new YtDlpSong(this, entry, options)),
          id: String(info.id ?? info.webpage_url ?? url),
          name: info.title ?? url,
          url: info.webpage_url ?? url,
          thumbnail: info.thumbnail ?? info.thumbnails?.[0]?.url,
        },
        options,
      );
    }

    return new YtDlpSong(this, info, options);
  }

  override async getStreamURL(song: Song) {
    if (!song.url) {
      throw new DisTubeError(
        'YTDLP_PLUGIN_INVALID_SONG',
        'Cannot get stream url from invalid song.',
      );
    }

    const info = await this.getInfo(song.url, 'ba/ba*');

    if (isPlaylist(info)) {
      throw new DisTubeError('YTDLP_ERROR', 'Cannot get stream URL of an entire playlist');
    }

    if (!info.url) {
      throw new DisTubeError('YTDLP_ERROR', 'yt-dlp did not return a stream URL');
    }

    return info.url;
  }

  override async searchSong<T>(query: string, options: ResolveOptions<T>) {
    const info = await this.getInfo(`ytsearch1:${query}`);
    const result = isPlaylist(info) ? info.entries[0] : info;

    return result ? new YtDlpSong(this, result, options) : null;
  }

  override getRelatedSongs() {
    return [];
  }

  private async getInfo(input: string, format?: string) {
    return getJson(input, {
      dumpSingleJson: true,
      noWarnings: true,
      preferFreeFormats: true,
      skipDownload: true,
      simulate: true,
      format,
    }).catch((error: unknown) => {
      const message = error instanceof Error ? error.message : String(error);
      throw new DisTubeError('YTDLP_ERROR', message);
    });
  }
}

class YtDlpSong<T = unknown> extends Song<T> {
  constructor(plugin: YtDlpExtractorPlugin, info: YtDlpInfo, options: ResolveOptions<T>) {
    super(
      {
        plugin,
        source: info.extractor ?? 'yt-dlp',
        playFromSource: true,
        id: String(info.id ?? info.webpage_url ?? info.original_url ?? info.url ?? 'unknown'),
        name: info.title ?? info.fulltitle ?? info.webpage_url ?? info.original_url ?? 'Unknown',
        url: info.webpage_url ?? info.original_url ?? info.url,
        isLive: info.is_live,
        thumbnail: info.thumbnail ?? info.thumbnails?.[0]?.url,
        duration: info.is_live ? 0 : info.duration,
        uploader: {
          name: info.uploader,
          url: info.uploader_url,
        },
      },
      options,
    );
  }
}
