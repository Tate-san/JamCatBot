import type { Queue } from 'distube';
import { t } from '../../i18n/t.js';

export function formatQueue(queue: Queue, language: string, limit = 10) {
  return queue.songs.slice(0, limit).map((song, index) =>
    t(language, 'music.queue.line', {
      position: index,
      title: song.name,
      duration: song.formattedDuration,
    }),
  );
}
