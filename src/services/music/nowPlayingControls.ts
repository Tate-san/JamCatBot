import { ComponentType, type Message } from "discord.js";
import { t } from "../../i18n/t.js";
import { BotError, toBotErrorCode } from "../commandUX/errors.js";
import type { MusicService } from "./musicService.js";
import {
  buildNowPlayingComponents,
  buildNowPlayingEmbed,
  NOW_PLAYING_NEXT_BUTTON_ID,
  NOW_PLAYING_PLAY_PAUSE_BUTTON_ID,
  NOW_PLAYING_PREVIOUS_BUTTON_ID,
  NOW_PLAYING_REFRESH_BUTTON_ID,
  NOW_PLAYING_STOP_BUTTON_ID
} from "./nowPlayingEmbed.js";

const CONTROL_TIMEOUT_MS = 15 * 60 * 1000;
const CONTROL_IDS = new Set([
  NOW_PLAYING_PREVIOUS_BUTTON_ID,
  NOW_PLAYING_PLAY_PAUSE_BUTTON_ID,
  NOW_PLAYING_NEXT_BUTTON_ID,
  NOW_PLAYING_STOP_BUTTON_ID,
  NOW_PLAYING_REFRESH_BUTTON_ID
]);

export function attachNowPlayingControls(options: {
  message: Message;
  guildId: string;
  language: string;
  musicService: MusicService;
  ownerId?: string;
}) {
  const collector = options.message.createMessageComponentCollector({
    componentType: ComponentType.Button,
    time: CONTROL_TIMEOUT_MS
  });

  collector.on("collect", async (button) => {
    if (!CONTROL_IDS.has(button.customId)) return;

    if (options.ownerId && button.user.id !== options.ownerId) {
      await button.reply({
        content: t(options.language, "common.notYourControl"),
        ephemeral: true
      });
      return;
    }

    try {
      if (button.customId === NOW_PLAYING_PREVIOUS_BUTTON_ID) {
        await options.musicService.previous(options.guildId);
      }

      if (button.customId === NOW_PLAYING_PLAY_PAUSE_BUTTON_ID) {
        await options.musicService.togglePause(options.guildId);
      }

      if (button.customId === NOW_PLAYING_NEXT_BUTTON_ID) {
        await options.musicService.skip(options.guildId);
      }

      if (button.customId === NOW_PLAYING_STOP_BUTTON_ID) {
        options.musicService.stop(options.guildId);
        collector.stop("stopped");

        await button.update({
          components: buildNowPlayingComponents(undefined)
        });
        return;
      }

      const queue = options.musicService.getQueue(options.guildId);
      const song = queue?.songs[0];

      if (!queue || !song) {
        collector.stop("empty");
        await button.update({
          content: t(options.language, "music.nowplaying.none"),
          embeds: [],
          components: []
        });
        return;
      }

      await button.update({
        embeds: [buildNowPlayingEmbed(queue, options.language, song)],
        components: buildNowPlayingComponents(queue)
      });
    } catch (error) {
      const code = toBotErrorCode(error);
      await button.reply({
        content:
          error instanceof BotError && code
            ? t(options.language, `errors.${code}`)
            : t(options.language, "common.error"),
        ephemeral: true
      });
    }
  });

  collector.on("end", async () => {
    const queue = options.musicService.getQueue(options.guildId);

    await options.message
      .edit({ components: buildNowPlayingComponents(queue, true) })
      .catch(() => undefined);
  });
}
