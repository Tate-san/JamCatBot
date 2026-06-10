import { Client } from "discord.js";
import { botIntents, botPartials } from "./intents.js";

export function createClient() {
  return new Client({
    intents: botIntents,
    partials: botPartials
  });
}
