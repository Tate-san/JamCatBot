import { createClient } from "./client/createClient.js";
import { registerEvents } from "./client/registerEvents.js";
import { env } from "./config/env.js";
import { registerApplicationCommands } from "./commands/registry.js";
import { initI18n } from "./i18n/i18n.js";
import { logger } from "./logging/logger.js";
import { initMusicService } from "./services/music/index.js";

export async function main() {
  logger.info("Starting JamCatBot");

  await initI18n();
  logger.info("i18n initialized");

  const client = createClient();
  initMusicService(client);
  registerEvents(client);
  logger.info("Discord client and services initialized");

  await registerApplicationCommands();

  logger.info("Logging in to Discord");
  await client.login(env.DISCORD_TOKEN);

  logger.info("Bot started");
}

main().catch((error) => {
  logger.fatal({ err: error }, "Fatal startup error");
  process.exit(1);
});
