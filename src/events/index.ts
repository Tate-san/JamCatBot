import { guildCreateEvent } from "./guildCreate.js";
import { interactionCreateEvent } from "./interactionCreate.js";
import { messageCreateEvent } from "./messageCreate.js";
import { readyEvent } from "./ready.js";
import { voiceStateUpdateEvent } from "./voiceStateUpdate.js";

export const events = [
  readyEvent,
  interactionCreateEvent,
  messageCreateEvent,
  guildCreateEvent,
  voiceStateUpdateEvent
];
