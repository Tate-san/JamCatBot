import { SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { getMusicService } from "../../services/music/index.js";
import type { LoopMode } from "../../services/music/musicService.js";

const loopModes = ["off", "on", "once"] as const;

function parseLoopMode(input: string | undefined) {
  if (!input) return undefined;

  const mode = input.toLowerCase();
  return loopModes.includes(mode as LoopMode) ? (mode as LoopMode) : undefined;
}

export const loopCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("loop")
    .setDescription("Set or toggle music loop mode")
    .addStringOption((option) =>
      option
        .setName("mode")
        .setDescription("Loop mode. Leave empty to toggle on/off.")
        .setRequired(false)
        .addChoices(
          { name: "Off", value: "off" },
          { name: "On", value: "on" },
          { name: "Once", value: "once" }
        )
    ),

  async execute({ interaction, language }) {
    const mode = parseLoopMode(interaction.options.getString("mode") ?? undefined);
    const selectedMode = getMusicService().setLoop(interaction.guildId!, mode);

    await interaction.reply(
      t(language, "music.loop.changed", { mode: selectedMode })
    );
  },

  async executePrefix({ message, language, prefix, args }) {
    const input = args[0];
    const mode = parseLoopMode(input);

    if (input && !mode) {
      await message.reply(`Usage: \`${prefix}loop [off|on|once]\``);
      return;
    }

    const selectedMode = getMusicService().setLoop(message.guildId!, mode);

    await message.reply(t(language, "music.loop.changed", { mode: selectedMode }));
  }
};
