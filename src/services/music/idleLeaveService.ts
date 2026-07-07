import type { DisTube } from 'distube';
import { logger } from '../../logging/logger.js';
import { guildSettingsService } from '../guildSettings/guildSettingsService.js';

export class IdleLeaveService {
  private timers = new Map<string, NodeJS.Timeout>();

  constructor(private distube: DisTube) {}

  async schedule(guildId: string) {
    this.cancel(guildId);

    const settings = await guildSettingsService.get(guildId);
    if (!settings.idleLeaveEnabled) return;

    const timer = setTimeout(() => {
      void this.leaveIfIdle(guildId);
    }, settings.idleLeaveSeconds * 1000);

    this.timers.set(guildId, timer);
  }

  cancel(guildId: string) {
    const timer = this.timers.get(guildId);

    if (timer) {
      clearTimeout(timer);
      this.timers.delete(guildId);
    }
  }

  private async leaveIfIdle(guildId: string) {
    try {
      const queue = this.distube.getQueue(guildId);

      if (queue?.playing) return;

      const voice = this.distube.voices.get(guildId);
      voice?.leave();

      logger.info({ guildId }, 'Left voice channel due to idle timeout');
    } catch (error) {
      logger.error({ err: error, guildId }, 'Idle leave failed');
    } finally {
      this.timers.delete(guildId);
    }
  }
}
