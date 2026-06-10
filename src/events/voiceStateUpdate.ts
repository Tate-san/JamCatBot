import { Events } from "discord.js";
import type { BotEvent } from "../client/registerEvents.js";

export const voiceStateUpdateEvent: BotEvent<Events.VoiceStateUpdate> = {
  name: Events.VoiceStateUpdate,
  execute() {
    // Reserved for future voice-channel UX, for example same-channel checks or
    // leaving immediately when the bot is alone and idle-leave is enabled.
  }
};
