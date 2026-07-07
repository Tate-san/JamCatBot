import { REST, Routes } from 'discord.js';
import { env } from '../config/env.js';
import { logger } from '../logging/logger.js';
import { slashCommands } from './index.js';

export async function registerApplicationCommands() {
  if (!env.REGISTER_SLASH_COMMANDS) {
    logger.info('Application command registration disabled');
    return;
  }

  const rest = new REST({
    version: '10',
    timeout: 15_000,
  }).setToken(env.DISCORD_TOKEN);

  const body = slashCommands.map((command) => command.data.toJSON());

  if (env.DISCORD_GUILD_ID) {
    logger.info(
      {
        count: body.length,
        guildId: env.DISCORD_GUILD_ID,
      },
      'Registering guild application commands',
    );

    await rest.put(Routes.applicationGuildCommands(env.DISCORD_CLIENT_ID, env.DISCORD_GUILD_ID), {
      body,
    });
    logger.info(
      {
        count: body.length,
        guildId: env.DISCORD_GUILD_ID,
      },
      'Registered guild application commands',
    );
    return;
  }

  logger.info({ count: body.length }, 'Registering global application commands');
  await rest.put(Routes.applicationCommands(env.DISCORD_CLIENT_ID), { body });
  logger.info({ count: body.length }, 'Registered global application commands');
}
