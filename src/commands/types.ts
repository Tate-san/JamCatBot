import type {
  ChatInputCommandInteraction,
  Message,
  PermissionResolvable
} from "discord.js";

export type CommandData = {
  name: string;
  toJSON(): unknown;
};

export type CommandContext = {
  interaction: ChatInputCommandInteraction;
  language: string;
};

export type PrefixCommandContext = {
  message: Message;
  language: string;
  prefix: string;
  args: string[];
  rawArgs: string;
};

export type BotCommand = {
  /**
   * Required only for prefix-only commands. Slash/both commands use data.name.
   */
  name?: string;
  /** Alternative prefix command names. */
  aliases?: string[];
  /** Add this to make the command available as a slash command. */
  data?: CommandData;
  requiredBotPermissions?: PermissionResolvable[];
  requiredUserPermissions?: PermissionResolvable[];
  /** Slash-only: defer the interaction before execute(). */
  defer?: boolean;
  /** Add this to make the command available as a slash command. */
  execute?(ctx: CommandContext): Promise<void>;
  /** Add this to make the command available as a prefix command. */
  executePrefix?(ctx: PrefixCommandContext): Promise<void>;
};

export type SlashBotCommand = BotCommand & {
  data: CommandData;
  execute(ctx: CommandContext): Promise<void>;
};

export type PrefixBotCommand = BotCommand & {
  executePrefix(ctx: PrefixCommandContext): Promise<void>;
};

export function defineCommand<T extends BotCommand>(command: T) {
  return command;
}
