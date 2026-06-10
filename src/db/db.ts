import Database from "better-sqlite3";
import { drizzle } from "drizzle-orm/better-sqlite3";
import { dirname } from "node:path";
import { mkdirSync } from "node:fs";
import { env } from "../config/env.js";
import * as schema from "./schema.js";

const databasePath = env.DATABASE_URL.replace(/^file:/, "");
mkdirSync(dirname(databasePath), { recursive: true });

const sqlite = new Database(databasePath);
sqlite.pragma("journal_mode = WAL");
sqlite.exec(`
  CREATE TABLE IF NOT EXISTS guild_settings (
    guild_id TEXT PRIMARY KEY NOT NULL,
    prefix TEXT NOT NULL DEFAULT '!',
    language TEXT NOT NULL DEFAULT 'en',
    music_volume INTEGER NOT NULL DEFAULT 50,
    idle_leave_enabled INTEGER NOT NULL DEFAULT 1,
    idle_leave_seconds INTEGER NOT NULL DEFAULT 300,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
  );

  CREATE TABLE IF NOT EXISTS command_permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    guild_id TEXT NOT NULL,
    command_name TEXT NOT NULL,
    allowed_role_id TEXT,
    denied_role_id TEXT,
    allowed_user_id TEXT,
    denied_user_id TEXT
  );
`);

export const db = drizzle(sqlite, { schema });
