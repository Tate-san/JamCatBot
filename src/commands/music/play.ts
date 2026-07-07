import { SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { ProgressMessage } from "../../services/commandUX/progressMessage.js";
import { guildSettingsService } from "../../services/guildSettings/guildSettingsService.js";
import { getMusicService } from "../../services/music/index.js";

export const playCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("play")
    .setDescription("Play or queue a song")
    .addStringOption((option) =>
      option
        .setName("query")
        .setDescription("Song name, URL, Spotify link, YouTube link, etc.")
        .setRequired(true)
    ),
  defer: true,

  async execute({ interaction, language }) {
    const query = interaction.options.getString("query", true);
    const settings = await guildSettingsService.get(interaction.guildId!);
    const progress = new ProgressMessage(interaction);
    const musicService = getMusicService();

    await progress.set(t(language, "music.play.searching", { query }));

    const result = await musicService.play({
      interaction,
      query,
      volume: settings.musicVolume,
      onProgress: (playlist) =>
        progress.set(
          t(
            language,
            playlist.done
              ? "music.play.playlistQueued"
              : "music.play.playlistProgress",
            playlist
          )
        )
    });

    if (result.kind === "playlist") return;

    const queue = musicService.getQueue(interaction.guildId!);
    const currentSong = queue?.songs.at(-1) ?? queue?.songs[0];

    await progress.set(
      currentSong
        ? t(language, "music.play.queued", { title: currentSong.name })
        : t(language, "music.play.playing", { title: query })
    );
  },

  async executePrefix({ message, language, prefix, rawArgs }) {
    const query = rawArgs.trim();
    if (!query) {
      await message.reply(`Usage: \`${prefix}play <query>\``);
      return;
    }

    const settings = await guildSettingsService.get(message.guildId!);
    const musicService = getMusicService();
    const progress = await message.reply(
      t(language, "music.play.searching", { query })
    );

    const result = await musicService.playFromMessage({
      message,
      query,
      volume: settings.musicVolume,
      onProgress: (playlist) =>
        progress.edit(
          t(
            language,
            playlist.done
              ? "music.play.playlistQueued"
              : "music.play.playlistProgress",
            playlist
          )
        )
    });

    if (result.kind === "playlist") return;

    const queue = musicService.getQueue(message.guildId!);
    const currentSong = queue?.songs.at(-1) ?? queue?.songs[0];

    await progress.edit(
      currentSong
        ? t(language, "music.play.queued", { title: currentSong.name })
        : t(language, "music.play.playing", { title: query })
    );
  }
};
