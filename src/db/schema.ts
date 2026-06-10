import { integer, sqliteTable, text } from "drizzle-orm/sqlite-core";

export const guildSettings = sqliteTable("guild_settings", {
  guildId: text("guild_id").primaryKey(),
  prefix: text("prefix").notNull().default("!"),
  language: text("language").notNull().default("en"),
  musicVolume: integer("music_volume").notNull().default(50),
  idleLeaveEnabled: integer("idle_leave_enabled", { mode: "boolean" })
    .notNull()
    .default(true),
  idleLeaveSeconds: integer("idle_leave_seconds").notNull().default(300),
  createdAt: integer("created_at", { mode: "timestamp_ms" }).notNull(),
  updatedAt: integer("updated_at", { mode: "timestamp_ms" }).notNull()
});

export const commandPermissions = sqliteTable("command_permissions", {
  id: integer("id").primaryKey({ autoIncrement: true }),
  guildId: text("guild_id").notNull(),
  commandName: text("command_name").notNull(),
  allowedRoleId: text("allowed_role_id"),
  deniedRoleId: text("denied_role_id"),
  allowedUserId: text("allowed_user_id"),
  deniedUserId: text("denied_user_id")
});
