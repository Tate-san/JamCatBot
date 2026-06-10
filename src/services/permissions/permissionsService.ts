import type { ChatInputCommandInteraction, Message } from "discord.js";
import { BotError } from "../commandUX/errors.js";
import type { BotCommand } from "../../commands/types.js";

export function assertCommandPermissions(
  interaction: ChatInputCommandInteraction,
  command: BotCommand
) {
  if (
    command.requiredUserPermissions?.length &&
    !interaction.memberPermissions?.has(command.requiredUserPermissions)
  ) {
    throw new BotError("NO_PERMISSION");
  }

  if (
    command.requiredBotPermissions?.length &&
    !interaction.appPermissions?.has(command.requiredBotPermissions)
  ) {
    throw new BotError("NO_PERMISSION");
  }
}

export function assertPrefixCommandPermissions(message: Message, command: BotCommand) {
  if (
    command.requiredUserPermissions?.length &&
    !message.member?.permissions.has(command.requiredUserPermissions)
  ) {
    throw new BotError("NO_PERMISSION");
  }

  const me = message.guild?.members.me;
  const botPermissions =
    me && "permissionsFor" in message.channel
      ? message.channel.permissionsFor(me)
      : null;

  if (
    command.requiredBotPermissions?.length &&
    !botPermissions?.has(command.requiredBotPermissions)
  ) {
    throw new BotError("NO_PERMISSION");
  }
}
