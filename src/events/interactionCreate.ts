import { Events } from "discord.js";
import type { BotEvent } from "../client/registerEvents.js";
import { slashCommandMap } from "../commands/index.js";
import { t } from "../i18n/t.js";
import { logger } from "../logging/logger.js";
import { guildSettingsService } from "../services/guildSettings/guildSettingsService.js";
import { assertCommandPermissions } from "../services/permissions/permissionsService.js";
import { BotError, toBotErrorCode } from "../services/commandUX/errors.js";

export const interactionCreateEvent: BotEvent<Events.InteractionCreate> = {
  name: Events.InteractionCreate,

  async execute(interaction) {
    if (!interaction.isChatInputCommand()) return;

    if (!interaction.guildId) {
      await interaction.reply({
        content: t("en", "common.guildOnly"),
        ephemeral: true
      });
      return;
    }

    const command = slashCommandMap.get(interaction.commandName);

    if (!command) {
      await interaction.reply({
        content: t("en", "common.unknownCommand"),
        ephemeral: true
      });
      return;
    }

    const settings = await guildSettingsService.get(interaction.guildId);
    const language = settings.language;
    const startedAt = Date.now();

    try {
      logger.info(
        {
          guildId: interaction.guildId,
          userId: interaction.user.id,
          command: interaction.commandName
        },
        "Executing command"
      );

      assertCommandPermissions(interaction, command);

      if (command.defer) {
        await interaction.deferReply();
      }

      await command.execute({ interaction, language });

      logger.info(
        {
          guildId: interaction.guildId,
          userId: interaction.user.id,
          command: interaction.commandName,
          durationMs: Date.now() - startedAt
        },
        "Command completed"
      );
    } catch (error) {
      logger.error(
        {
          err: error,
          guildId: interaction.guildId,
          userId: interaction.user.id,
          command: interaction.commandName,
          durationMs: Date.now() - startedAt
        },
        "Command failed"
      );

      const code = toBotErrorCode(error);
      const message =
        error instanceof BotError && code
          ? t(language, `errors.${code}`)
          : t(language, "common.error");

      if (interaction.deferred || interaction.replied) {
        await interaction.editReply({ content: message }).catch(() => undefined);
      } else {
        await interaction
          .reply({ content: message, ephemeral: true })
          .catch(() => undefined);
      }
    }
  }
};
