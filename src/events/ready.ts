import { Events } from "discord.js";
import type { BotEvent } from "../client/registerEvents.js";
import { logger } from "../logging/logger.js";

export const readyEvent: BotEvent<Events.ClientReady> = {
  name: Events.ClientReady,
  once: true,
  execute(client) {
    logger.info(
      { user: client.user.tag, guilds: client.guilds.cache.size },
      "Discord client ready"
    );
  }
};
