import { SlashCommandBuilder } from 'discord.js';
import type { BotCommand } from '../types.js';
import { sendPaginatedMessage } from '../../services/commandUX/pagination.js';
import { guildSettingsService } from '../../services/guildSettings/guildSettingsService.js';
import {
  buildHelpPage,
  findHelpPage,
  getHelpPageCount,
  getHelpPageLabel,
} from '../../services/help/helpView.js';

export const helpCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName('help')
    .setDescription('Show the command help menu')
    .addStringOption((option) =>
      option
        .setName('category')
        .setDescription('Optional category to open')
        .setRequired(false)
        .addChoices(
          {
            name: 'Music',
            value: 'music',
          },
          {
            name: 'Settings',
            value: 'settings',
          },
          {
            name: 'Fun',
            value: 'fun',
          },
          {
            name: 'General',
            value: 'general',
          },
        ),
    ),

  async execute({ interaction, language }) {
    const category = interaction.options.getString('category') ?? undefined;
    const settings = await guildSettingsService.get(interaction.guildId!);

    await sendPaginatedMessage({
      ownerId: interaction.user.id,
      language,
      style: 'buttons-dropdown',
      initialPage: findHelpPage(category),
      getTotalPages: getHelpPageCount,
      getPageLabel: getHelpPageLabel,
      buildPage: ({ page, totalPages }) => buildHelpPage(settings.prefix, page, totalPages),
      send: async (payload) => {
        await interaction.reply({
          ...payload,
          ephemeral: true,
        });
        return interaction.fetchReply();
      },
    });
  },

  async executePrefix({ message, language, prefix, args }) {
    await sendPaginatedMessage({
      ownerId: message.author.id,
      language,
      style: 'buttons-dropdown',
      initialPage: findHelpPage(args[0]),
      getTotalPages: getHelpPageCount,
      getPageLabel: getHelpPageLabel,
      buildPage: ({ page, totalPages }) => buildHelpPage(prefix, page, totalPages),
      send: (payload) => message.reply(payload),
    });
  },
};
