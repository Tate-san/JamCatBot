import { Events } from "discord.js";
import type { BotEvent } from "../client/registerEvents.js";
import { prefixCommandMap } from "../commands/index.js";
import { t } from "../i18n/t.js";
import { logger } from "../logging/logger.js";
import { BotError, toBotErrorCode } from "../services/commandUX/errors.js";
import { guildSettingsService } from "../services/guildSettings/guildSettingsService.js";
import { assertPrefixCommandPermissions } from "../services/permissions/permissionsService.js";

function parseArgs(input: string) {
  const args: string[] = [];
  const pattern = /"([^"]*)"|'([^']*)'|(\S+)/g;
  let match: RegExpExecArray | null;

  while ((match = pattern.exec(input))) {
    args.push(match[1] ?? match[2] ?? match[3] ?? "");
  }

  return args;
}

export const messageCreateEvent: BotEvent<Events.MessageCreate> = {
  name: Events.MessageCreate,

  async execute(message) {
    if (message.author.bot || !message.guildId) return;

    const settings = await guildSettingsService.get(message.guildId);
    if (!message.content.startsWith(settings.prefix)) return;

    const input = message.content.slice(settings.prefix.length).trim();
    if (!input) return;

    const commandName = input.split(/\s+/, 1)[0]?.toLowerCase();
    if (!commandName) return;

    const command = prefixCommandMap.get(commandName);

    if (!command) {
      await message.reply(t(settings.language, "common.unknownCommand"));
      return;
    }

    const rawArgs = input.slice(commandName.length).trim();
    const args = parseArgs(rawArgs);
    const startedAt = Date.now();

    try {
      logger.info(
        {
          guildId: message.guildId,
          userId: message.author.id,
          command: commandName,
          source: "prefix"
        },
        "Executing command"
      );

      assertPrefixCommandPermissions(message, command);

      await command.executePrefix({
        message,
        language: settings.language,
        prefix: settings.prefix,
        args,
        rawArgs
      });

      logger.info(
        {
          guildId: message.guildId,
          userId: message.author.id,
          command: commandName,
          source: "prefix",
          durationMs: Date.now() - startedAt
        },
        "Command completed"
      );
    } catch (error) {
      logger.error(
        {
          err: error,
          guildId: message.guildId,
          userId: message.author.id,
          command: commandName,
          source: "prefix",
          durationMs: Date.now() - startedAt
        },
        "Command failed"
      );

      const code = toBotErrorCode(error);
      const response =
        error instanceof BotError && code
          ? t(settings.language, `errors.${code}`)
          : t(settings.language, "common.error");

      await message.reply(response).catch(() => undefined);
    }
  }
};
