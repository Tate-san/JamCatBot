import type { Client } from 'discord.js';
import { Events as DisTubeEvents, type Queue, type Song } from 'distube';
import { logger } from '../../logging/logger.js';
import { guildSettingsService } from '../guildSettings/guildSettingsService.js';
import { createDisTube } from './distube.js';
import { IdleLeaveService } from './idleLeaveService.js';
import { MusicService } from './musicService.js';
import { attachNowPlayingControls } from './nowPlayingControls.js';
import { buildNowPlayingComponents, buildNowPlayingEmbed } from './nowPlayingEmbed.js';

let musicService: MusicService | undefined;
let idleLeaveService: IdleLeaveService | undefined;

export function initMusicService(client: Client) {
  const distube = createDisTube(client);
  musicService = new MusicService(distube);
  idleLeaveService = new IdleLeaveService(distube);

  distube.on(DisTubeEvents.PLAY_SONG, (queue: Queue, song: Song) => {
    idleLeaveService?.cancel(queue.id);

    void (async () => {
      if (!queue.textChannel) return;

      const settings = await guildSettingsService.get(queue.id);
      const playerMessage = await queue.textChannel.send({
        embeds: [buildNowPlayingEmbed(queue, settings.language, song)],
        components: buildNowPlayingComponents(queue),
      });

      attachNowPlayingControls({
        message: playerMessage,
        guildId: queue.id,
        language: settings.language,
        musicService: musicService!,
      });
    })().catch((error) => {
      logger.warn({ err: error, guildId: queue.id }, 'Failed to send now playing player');
    });
  });

  distube.on(DisTubeEvents.ADD_SONG, (queue: Queue) => {
    idleLeaveService?.cancel(queue.id);
  });

  distube.on(DisTubeEvents.FINISH, (queue: Queue) => {
    void idleLeaveService?.schedule(queue.id);
  });

  distube.on(DisTubeEvents.DISCONNECT, (queue: Queue) => {
    idleLeaveService?.cancel(queue.id);
  });

  distube.on(DisTubeEvents.ERROR, (error: Error, queue?: Queue) => {
    logger.error({ err: error, guildId: queue?.id }, 'DisTube error');
  });

  distube.on(DisTubeEvents.DEBUG, (message: string) => {
    logger.debug({ message }, 'DisTube debug');
  });

  distube.on(DisTubeEvents.FFMPEG_DEBUG, (message: string) => {
    logger.debug({ message }, 'FFmpeg debug');
  });

  return musicService;
}

export function getMusicService() {
  if (!musicService) {
    throw new Error('Music service has not been initialized.');
  }

  return musicService;
}
