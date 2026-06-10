import "dotenv/config";
import { z } from "zod";

const schema = z.object({
  DISCORD_TOKEN: z.string().min(1),
  DISCORD_CLIENT_ID: z.string().min(1),
  DISCORD_GUILD_ID: z.string().min(1).optional(),
  DATABASE_URL: z.string().default("data/jamcat.sqlite"),
  NODE_ENV: z.enum(["development", "production", "test"]).default("development"),
  LOG_LEVEL: z.string().default("info"),
  REGISTER_SLASH_COMMANDS: z
    .string()
    .optional()
    .transform((value) => value !== "false"),
  YTDLP_PATH: z.string().default("yt-dlp"),
  FFMPEG_PATH: z.string().optional(),
  SPOTIFY_CLIENT_ID: z.string().min(1).optional(),
  SPOTIFY_CLIENT_SECRET: z.string().min(1).optional(),
  SPOTIFY_TOP_TRACKS_COUNTRY: z.string().regex(/^[A-Z]{2}$/).optional()
});

export const env = schema.parse(process.env);
