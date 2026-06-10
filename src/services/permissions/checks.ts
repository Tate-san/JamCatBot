import {
  GuildMember,
  PermissionFlagsBits,
  type VoiceBasedChannel
} from "discord.js";
import { BotError } from "../commandUX/errors.js";

export function assertMemberInVoice(member: GuildMember) {
  const channel = member.voice.channel;

  if (!channel) {
    throw new BotError("USER_NOT_IN_VOICE");
  }

  return channel;
}

export function assertBotCanUseVoice(channel: VoiceBasedChannel, me: GuildMember) {
  const permissions = channel.permissionsFor(me);

  if (!permissions?.has(PermissionFlagsBits.Connect)) {
    throw new BotError("BOT_CANNOT_CONNECT");
  }

  if (!permissions?.has(PermissionFlagsBits.Speak)) {
    throw new BotError("BOT_CANNOT_SPEAK");
  }
}
