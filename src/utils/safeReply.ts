import type {
  ChatInputCommandInteraction,
  InteractionEditReplyOptions,
  InteractionReplyOptions
} from "discord.js";

export async function safeReply(
  interaction: ChatInputCommandInteraction,
  options: string | InteractionReplyOptions
) {
  const replyPayload = typeof options === "string" ? { content: options } : options;
  const editPayload = replyPayload as InteractionEditReplyOptions;

  if (interaction.deferred || interaction.replied) {
    await interaction.editReply(editPayload).catch(() => undefined);
  } else {
    await interaction.reply(replyPayload).catch(() => undefined);
  }
}
