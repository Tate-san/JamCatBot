import { chudCommand } from "./fun/chud.js";
import { jamCommand } from "./fun/jam.js";
import { shitpostCommand } from "./fun/shitpost.js";
import { sixSevenCommand } from "./fun/sixseven.js";
import { helpCommand } from "./general/help.js";
import { leaveCommand } from "./music/leave.js";
import { loopCommand } from "./music/loop.js";
import { moveCommand } from "./music/move.js";
import { nowPlayingCommand } from "./music/nowplaying.js";
import { playCommand } from "./music/play.js";
import { queueCommand } from "./music/queue.js";
import { seekCommand } from "./music/seek.js";
import { skipCommand } from "./music/skip.js";
import { stopCommand } from "./music/stop.js";
import { volumeCommand } from "./music/volume.js";
import { settingsCommand } from "./settings/settings.js";
import type { BotCommand, PrefixBotCommand, SlashBotCommand } from "./types.js";

export const commands: BotCommand[] = [
  helpCommand,
  playCommand,
  queueCommand,
  nowPlayingCommand,
  skipCommand,
  stopCommand,
  seekCommand,
  moveCommand,
  volumeCommand,
  loopCommand,
  leaveCommand,
  shitpostCommand,
  jamCommand,
  sixSevenCommand,
  chudCommand,
  settingsCommand
];

function getCommandName(command: BotCommand) {
  const name = command.name ?? command.data?.name;

  if (!name) {
    throw new Error("Command must define either data.name or name.");
  }

  return name;
}

function isSlashCommand(command: BotCommand): command is SlashBotCommand {
  return Boolean(command.data && command.execute);
}

function isPrefixCommand(command: BotCommand): command is PrefixBotCommand {
  return Boolean(command.executePrefix);
}

export const slashCommands = commands.filter(isSlashCommand);
export const prefixCommands = commands.filter(isPrefixCommand);

export const slashCommandMap = new Map(
  slashCommands.map((command) => [command.data.name, command])
);

export const prefixCommandMap = new Map<string, PrefixBotCommand>();

for (const command of prefixCommands) {
  prefixCommandMap.set(getCommandName(command).toLowerCase(), command);

  for (const alias of command.aliases ?? []) {
    prefixCommandMap.set(alias.toLowerCase(), command);
  }
}
