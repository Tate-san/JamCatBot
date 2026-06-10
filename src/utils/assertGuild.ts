import type { ChatInputCommandInteraction } from "discord.js";
import { BotError } from "../services/commandUX/errors.js";

export function assertGuild(interaction: ChatInputCommandInteraction) {
  if (!interaction.guild || !interaction.guildId) {
    throw new BotError("GUILD_ONLY");
  }

  return interaction.guild;
}
