import { PermissionFlagsBits, SlashCommandBuilder } from "discord.js";
import type { BotCommand } from "../types.js";
import { t } from "../../i18n/t.js";
import { ProgressMessage } from "../../services/commandUX/progressMessage.js";
import { shitpostService } from "../../services/shitpost/shitpostService.js";

export const shitpostCommand: BotCommand = {
  data: new SlashCommandBuilder()
    .setName("shitpost")
    .setDescription("Download and reupload media, compressing it if needed")
    .addStringOption((option) =>
      option.setName("url").setDescription("Media URL").setRequired(true)
    ),
  requiredBotPermissions: [PermissionFlagsBits.AttachFiles],
  defer: true,

  async execute({ interaction, language }) {
    const url = interaction.options.getString("url", true);
    const progress = new ProgressMessage(interaction);

    const processed = await shitpostService.process(url, async (step) => {
      await progress.set(t(language, `shitpost.${step}`));
    });

    try {
      await progress.set(t(language, "shitpost.uploading"));
      await interaction.editReply({
        content: t(language, "shitpost.done"),
        files: [processed.attachment]
      });
    } finally {
      await processed.cleanup().catch(() => undefined);
    }
  },

  async executePrefix({ message, language, prefix, args }) {
    const url = args[0];
    if (!url) {
      await message.reply(`Usage: \`${prefix}shitpost <url>\``);
      return;
    }

    const progress = await message.reply(t(language, "shitpost.downloading"));
    const processed = await shitpostService.process(url, async (step) => {
      await progress.edit(t(language, `shitpost.${step}`));
    });

    try {
      await progress.edit(t(language, "shitpost.uploading"));
      await progress.edit({
        content: t(language, "shitpost.done"),
        files: [processed.attachment]
      });
    } finally {
      await processed.cleanup().catch(() => undefined);
    }
  }
};
