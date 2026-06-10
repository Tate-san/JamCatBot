import { Events } from "discord.js";
import type { BotEvent } from "../client/registerEvents.js";
import { logger } from "../logging/logger.js";
import { guildSettingsService } from "../services/guildSettings/guildSettingsService.js";

export const guildCreateEvent: BotEvent<Events.GuildCreate> = {
  name: Events.GuildCreate,

  async execute(guild) {
    await guildSettingsService.get(guild.id);
    logger.info({ guildId: guild.id, name: guild.name }, "Joined guild");
  }
};
