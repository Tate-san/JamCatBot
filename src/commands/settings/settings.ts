import { PermissionFlagsBits, SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { sendPaginatedMessage } from "../../services/commandUX/pagination.js";
import { guildSettingsService } from "../../services/guildSettings/guildSettingsService.js";
import {
  buildSettingsPage,
  getSettingsPageCount
} from "../../services/guildSettings/settingsView.js";

export const settingsCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("settings")
    .setDescription("Manage server settings")
    .setDefaultMemberPermissions(PermissionFlagsBits.ManageGuild)
    .addSubcommand((sub) =>
      sub.setName("show").setDescription("Show current settings")
    )
    .addSubcommandGroup((group) =>
      group
        .setName("prefix")
        .setDescription("Manage prefix")
        .addSubcommand((sub) =>
          sub
            .setName("set")
            .setDescription("Set prefix")
            .addStringOption((option) =>
              option
                .setName("value")
                .setDescription("New prefix")
                .setRequired(true)
                .setMaxLength(5)
            )
        )
    )
    .addSubcommandGroup((group) =>
      group
        .setName("language")
        .setDescription("Manage language")
        .addSubcommand((sub) =>
          sub
            .setName("set")
            .setDescription("Set language")
            .addStringOption((option) =>
              option
                .setName("value")
                .setDescription("Language code")
                .setRequired(true)
                .addChoices(
                  { name: "English", value: "en" },
                  { name: "Sassy", value: "sassy" }
                )
            )
        )
    )
    .addSubcommandGroup((group) =>
      group
        .setName("idle-leave")
        .setDescription("Manage idle voice leave")
        .addSubcommand((sub) =>
          sub
            .setName("enable")
            .setDescription("Enable idle leave")
            .addIntegerOption((option) =>
              option
                .setName("seconds")
                .setDescription("Seconds before leaving")
                .setRequired(false)
                .setMinValue(30)
                .setMaxValue(3600)
            )
        )
        .addSubcommand((sub) =>
          sub.setName("disable").setDescription("Disable idle leave")
        )
    ),
  requiredUserPermissions: [PermissionFlagsBits.ManageGuild],

  async execute({ interaction, language }) {
    const group = interaction.options.getSubcommandGroup(false);
    const subcommand = interaction.options.getSubcommand();

    if (!group && subcommand === "show") {
      await sendPaginatedMessage({
        ownerId: interaction.user.id,
        language,
        style: "buttons-dropdown",
        getTotalPages: async () =>
          getSettingsPageCount(
            await guildSettingsService.get(interaction.guildId!)
          ),
        buildPage: async ({ page, totalPages }) =>
          buildSettingsPage(
            await guildSettingsService.get(interaction.guildId!),
            language,
            page,
            totalPages
          ),
        send: async (payload) => {
          await interaction.reply({
            ...payload,
            ephemeral: true
          });
          return interaction.fetchReply();
        }
      });
      return;
    }

    if (group === "prefix" && subcommand === "set") {
      const prefix = interaction.options.getString("value", true);
      await guildSettingsService.update(interaction.guildId!, { prefix });
      await interaction.reply({
        content: t(language, "settings.prefix.changed", { prefix }),
        ephemeral: true
      });
      return;
    }

    if (group === "language" && subcommand === "set") {
      const newLanguage = interaction.options.getString("value", true);
      await guildSettingsService.update(interaction.guildId!, {
        language: newLanguage
      });
      await interaction.reply({
        content: t(newLanguage, "settings.language.changed", {
          language: newLanguage
        }),
        ephemeral: true
      });
      return;
    }

    if (group === "idle-leave" && subcommand === "enable") {
      const seconds = interaction.options.getInteger("seconds") ?? 300;
      await guildSettingsService.update(interaction.guildId!, {
        idleLeaveEnabled: true,
        idleLeaveSeconds: seconds
      });
      await interaction.reply({
        content: t(language, "settings.idleLeave.enabled", { seconds }),
        ephemeral: true
      });
      return;
    }

    if (group === "idle-leave" && subcommand === "disable") {
      await guildSettingsService.update(interaction.guildId!, {
        idleLeaveEnabled: false
      });
      await interaction.reply({
        content: t(language, "settings.idleLeave.disabled"),
        ephemeral: true
      });
    }
  },

  async executePrefix({ message, language, prefix: commandPrefix, args }) {
    const group = args[0]?.toLowerCase();
    const subcommand = args[1]?.toLowerCase();

    if (!group || group === "show") {
      await sendPaginatedMessage({
        ownerId: message.author.id,
        language,
        style: "buttons-dropdown",
        getTotalPages: async () =>
          getSettingsPageCount(await guildSettingsService.get(message.guildId!)),
        buildPage: async ({ page, totalPages }) =>
          buildSettingsPage(
            await guildSettingsService.get(message.guildId!),
            language,
            page,
            totalPages
          ),
        send: (payload) => message.reply(payload)
      });
      return;
    }

    if (group === "prefix") {
      const prefix = subcommand === "set" ? args[2] : args[1];
      if (!prefix) {
        await message.reply(
          `Usage: \`${commandPrefix}settings prefix set <prefix>\``
        );
        return;
      }

      await guildSettingsService.update(message.guildId!, { prefix });
      await message.reply(t(language, "settings.prefix.changed", { prefix }));
      return;
    }

    if (group === "language") {
      const newLanguage = subcommand === "set" ? args[2] : args[1];
      if (!newLanguage) {
        await message.reply(
          `Usage: \`${commandPrefix}settings language set <language>\``
        );
        return;
      }

      await guildSettingsService.update(message.guildId!, {
        language: newLanguage
      });
      await message.reply(
        t(newLanguage, "settings.language.changed", { language: newLanguage })
      );
      return;
    }

    if (group === "idle-leave") {
      if (subcommand === "enable") {
        const seconds = args[2] ? Number(args[2]) : 300;
        if (!Number.isInteger(seconds) || seconds < 30 || seconds > 3600) {
          await message.reply(
            `Usage: \`${commandPrefix}settings idle-leave enable [30-3600]\``
          );
          return;
        }

        await guildSettingsService.update(message.guildId!, {
          idleLeaveEnabled: true,
          idleLeaveSeconds: seconds
        });
        await message.reply(
          t(language, "settings.idleLeave.enabled", { seconds })
        );
        return;
      }

      if (subcommand === "disable") {
        await guildSettingsService.update(message.guildId!, {
          idleLeaveEnabled: false
        });
        await message.reply(t(language, "settings.idleLeave.disabled"));
        return;
      }
    }

    await message.reply(
      `Usage: \`${commandPrefix}settings show\`, \`${commandPrefix}settings prefix set <prefix>\`, \`${commandPrefix}settings language set <language>\`, or \`${commandPrefix}settings idle-leave enable|disable\``
    );
  }
};
