import {
  GuildMember,
  type ChatInputCommandInteraction,
  type GuildTextBasedChannel,
  type Message,
  type VoiceBasedChannel
} from "discord.js";
import { RepeatMode, type DisTube, type Queue } from "distube";
import { BotError } from "../commandUX/errors.js";
import {
  assertBotCanUseVoice,
  assertMemberInVoice
} from "../permissions/checks.js";

export type LoopMode = "off" | "on" | "once";

export class MusicService {
  constructor(private distube: DisTube) {}

  async play(options: {
    interaction: ChatInputCommandInteraction;
    query: string;
    volume: number;
  }) {
    const member = options.interaction.member as GuildMember;

    await this.playForMember({
      guildId: options.interaction.guildId!,
      member,
      me: options.interaction.guild?.members.me,
      query: options.query,
      volume: options.volume,
      textChannel: (options.interaction.channel ?? undefined) as
        | GuildTextBasedChannel
        | undefined,
      metadata: {
        interactionId: options.interaction.id
      }
    });
  }

  async playFromMessage(options: {
    message: Message;
    query: string;
    volume: number;
  }) {
    const member = options.message.member as GuildMember | null;
    if (!member) throw new BotError("GUILD_ONLY");

    await this.playForMember({
      guildId: options.message.guildId!,
      member,
      me: options.message.guild?.members.me,
      query: options.query,
      volume: options.volume,
      textChannel: options.message.channel as GuildTextBasedChannel,
      metadata: {
        messageId: options.message.id
      }
    });
  }

  private async playForMember(options: {
    guildId: string;
    member: GuildMember;
    me: GuildMember | null | undefined;
    query: string;
    volume: number;
    textChannel: GuildTextBasedChannel | undefined;
    metadata: Record<string, string>;
  }) {
    const voiceChannel = assertMemberInVoice(options.member);

    if (options.me) {
      assertBotCanUseVoice(voiceChannel, options.me);
    }

    await this.distube.play(voiceChannel as VoiceBasedChannel, options.query, {
      member: options.member,
      textChannel: options.textChannel,
      metadata: options.metadata
    });

    const queue = this.getQueue(options.guildId);
    if (queue) {
      queue.setVolume(options.volume);
    }
  }

  getQueue(guildId: string): Queue | undefined {
    return this.distube.getQueue(guildId);
  }

  async skip(guildId: string) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError("NO_QUEUE");

    if (queue.songs.length <= 1) {
      queue.stop();
      return;
    }

    return queue.skip();
  }

  async previous(guildId: string) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError("NO_QUEUE");

    return queue.previous();
  }

  async togglePause(guildId: string) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError("NO_QUEUE");

    if (queue.paused) {
      await queue.resume();
      return false;
    }

    await queue.pause();
    return true;
  }

  setVolume(guildId: string, volume: number) {
    const queue = this.getQueue(guildId);
    queue?.setVolume(volume);
  }

  setLoop(guildId: string, mode?: LoopMode) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError("NO_QUEUE");

    const nextMode: LoopMode =
      mode ?? (queue.repeatMode === RepeatMode.QUEUE ? "off" : "on");

    const repeatMode =
      nextMode === "off"
        ? RepeatMode.DISABLED
        : nextMode === "once"
          ? RepeatMode.SONG
          : RepeatMode.QUEUE;

    queue.setRepeatMode(repeatMode);
    return nextMode;
  }

  stop(guildId: string) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError("NO_QUEUE");

    queue.stop();
  }

  seek(guildId: string, seconds: number) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError("NO_QUEUE");

    queue.seek(seconds);
  }

  leave(guildId: string) {
    const queue = this.getQueue(guildId);

    if (queue) {
      queue.voice.leave();
      return;
    }

    const voice = this.distube.voices.get(guildId);
    if (voice) {
      voice.leave();
      return;
    }

    throw new BotError("NOT_CONNECTED");
  }

  move(guildId: string, from: number, to: number) {
    const queue = this.getQueue(guildId);
    if (!queue) throw new BotError("NO_QUEUE");

    const songs = queue.songs;
    const fromIndex = from - 1;
    const toIndex = to - 1;

    if (
      fromIndex <= 0 ||
      toIndex <= 0 ||
      fromIndex >= songs.length ||
      toIndex >= songs.length
    ) {
      throw new BotError("INVALID_POSITION");
    }

    const [item] = songs.splice(fromIndex, 1);
    if (!item) throw new BotError("INVALID_POSITION");

    songs.splice(toIndex, 0, item);
  }
}
