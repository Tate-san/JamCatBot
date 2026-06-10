import { eq } from "drizzle-orm";
import { db } from "../../db/db.js";
import { guildSettings } from "../../db/schema.js";
import {
  DEFAULT_GUILD_SETTINGS,
  type GuildSettingsPatch
} from "./defaultSettings.js";

export class GuildSettingsService {
  async get(guildId: string) {
    const existing = db
      .select()
      .from(guildSettings)
      .where(eq(guildSettings.guildId, guildId))
      .get();

    if (existing) return existing;

    const now = new Date();
    const created = {
      guildId,
      ...DEFAULT_GUILD_SETTINGS,
      createdAt: now,
      updatedAt: now
    };

    db.insert(guildSettings).values(created).onConflictDoNothing().run();

    return created;
  }

  async update(guildId: string, patch: GuildSettingsPatch) {
    await this.get(guildId);

    db.update(guildSettings)
      .set({
        ...patch,
        updatedAt: new Date()
      })
      .where(eq(guildSettings.guildId, guildId))
      .run();

    return this.get(guildId);
  }
}

export const guildSettingsService = new GuildSettingsService();
