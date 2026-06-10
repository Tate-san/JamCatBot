import { SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { getMusicService } from "../../services/music/index.js";
import { attachNowPlayingControls } from "../../services/music/nowPlayingControls.js";
import {
  buildNowPlayingComponents,
  buildNowPlayingEmbed
} from "../../services/music/nowPlayingEmbed.js";

export const nowPlayingCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("nowplaying")
    .setDescription("Show the current song"),

  async execute({ interaction, language }) {
    const queue = getMusicService().getQueue(interaction.guildId!);
    const song = queue?.songs[0];

    if (!queue || !song) {
      await interaction.reply(t(language, "music.nowplaying.none"));
      return;
    }

    await interaction.reply({
      embeds: [buildNowPlayingEmbed(queue, language, song)],
      components: buildNowPlayingComponents(queue)
    });
    const reply = await interaction.fetchReply();

    attachNowPlayingControls({
      message: reply,
      guildId: interaction.guildId!,
      language,
      musicService: getMusicService(),
      ownerId: interaction.user.id
    });
  },

  async executePrefix({ message, language }) {
    const queue = getMusicService().getQueue(message.guildId!);
    const song = queue?.songs[0];

    if (!queue || !song) {
      await message.reply(t(language, "music.nowplaying.none"));
      return;
    }

    const reply = await message.reply({
      embeds: [buildNowPlayingEmbed(queue, language, song)],
      components: buildNowPlayingComponents(queue)
    });

    attachNowPlayingControls({
      message: reply,
      guildId: message.guildId!,
      language,
      musicService: getMusicService(),
      ownerId: message.author.id
    });
  }
};
