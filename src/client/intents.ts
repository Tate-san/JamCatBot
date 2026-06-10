import { GatewayIntentBits, Partials } from "discord.js";

export const botIntents = [
  GatewayIntentBits.Guilds,
  GatewayIntentBits.GuildMessages,
  GatewayIntentBits.GuildVoiceStates,
  GatewayIntentBits.MessageContent
];

export const botPartials = [Partials.Channel, Partials.Message, Partials.Reaction];
