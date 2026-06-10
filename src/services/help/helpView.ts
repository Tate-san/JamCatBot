import { EmbedBuilder } from "discord.js";

type HelpCommandEntry = {
  usage: string;
  description: string;
  params?: string[];
  availability?: "both" | "slash" | "prefix";
};

type HelpCategory = {
  name: string;
  emoji: string;
  description: string;
  commands: HelpCommandEntry[];
};

const helpCategories: HelpCategory[] = [
  {
    name: "Music",
    emoji: "🎵",
    description: "Playback and queue controls.",
    commands: [
      {
        usage: "play <query>",
        description: "Play or queue a song from a search, YouTube URL, Spotify URL, or direct URL.",
        params: ["query: song name, URL, Spotify link, YouTube link, etc."]
      },
      {
        usage: "queue",
        description: "Show the current queue with paginated controls."
      },
      {
        usage: "nowplaying",
        description: "Show the current player with progress and playback controls."
      },
      {
        usage: "skip",
        description: "Skip the current song. If nothing is queued next, playback stops."
      },
      {
        usage: "stop",
        description: "Stop playback and clear the queue."
      },
      {
        usage: "seek <time>",
        description: "Seek to a timestamp in the current song.",
        params: ["time: seconds, mm:ss, or hh:mm:ss"]
      },
      {
        usage: "move <from> <to>",
        description: "Move a queued song to another position.",
        params: ["from: current queue position", "to: new queue position"]
      },
      {
        usage: "volume <1-100>",
        description: "Set the music volume and save it as the server default.",
        params: ["value: volume percentage from 1 to 100"]
      },
      {
        usage: "loop [off|on|once]",
        description: "Set loop mode. With no argument, toggles between queue loop and off.",
        params: ["off: disable loop", "on: loop the queue", "once: loop the current song"]
      },
      {
        usage: "leave",
        description: "Make the bot leave the voice channel."
      }
    ]
  },
  {
    name: "Settings",
    emoji: "⚙️",
    description: "Server configuration. Requires Manage Server.",
    commands: [
      {
        usage: "settings show",
        description: "Show server settings as a paginated list."
      },
      {
        usage: "settings prefix set <prefix>",
        description: "Change the prefix used for prefix commands.",
        params: ["prefix: new prefix, max 5 characters"]
      },
      {
        usage: "settings language set <language>",
        description: "Change the server language.",
        params: ["language: language code, currently en or sassy"]
      },
      {
        usage: "settings idle-leave enable [seconds]",
        description: "Enable automatic leave after the bot is idle.",
        params: ["seconds: optional timeout from 30 to 3600 seconds"]
      },
      {
        usage: "settings idle-leave disable",
        description: "Disable automatic idle leave."
      }
    ]
  },
  {
    name: "Fun",
    emoji: "🎭",
    description: "Media utilities.",
    commands: [
      {
        usage: "shitpost <url>",
        description: "Download and reupload media, compressing it if needed.",
        params: ["url: media URL to process"]
      },
      {
        usage: "jam",
        description: "Reply with the JamCat jammin gif.",
        availability: "prefix"
      },
      {
        usage: "sixseven",
        description: "Reply with the 6-7 cat gif.",
        availability: "prefix"
      },
      {
        usage: "chud",
        description: "Reply with the chud gif.",
        availability: "prefix"
      }
    ]
  },
  {
    name: "General",
    emoji: "📚",
    description: "Bot information.",
    commands: [
      {
        usage: "help",
        description: "Show this paginated help menu."
      }
    ]
  }
];

function clampPage(page: number) {
  return Math.min(Math.max(page, 0), helpCategories.length - 1);
}

function formatCommand(prefix: string, command: HelpCommandEntry) {
  const availability = command.availability ?? "both";
  const usages = [
    availability !== "prefix" ? `/${command.usage}` : undefined,
    availability !== "slash" ? `${prefix}${command.usage}` : undefined
  ]
    .filter(Boolean)
    .map((usage) => `\`${usage}\``)
    .join(" / ");
  const params = "params" in command && command.params?.length
    ? `\n  ${command.params.map((param) => `• ${param}`).join("\n  ")}`
    : "";

  return `- ${usages} — ${command.description}${params}`;
}

export function getHelpPageCount(): number {
  return helpCategories.length;
}

export function getHelpPageLabel(page: number): string {
  return helpCategories[clampPage(page)]?.name ?? "Help";
}

export function findHelpPage(category: string | undefined) {
  if (!category) return 0;

  const normalized = category.toLowerCase();
  const index = helpCategories.findIndex(
    (item) =>
      item.name.toLowerCase() === normalized ||
      item.name.toLowerCase().startsWith(normalized)
  );

  return index === -1 ? 0 : index;
}

export function buildHelpPage(
  prefix: string,
  page = 0,
  totalPages: number = getHelpPageCount()
) {
  const currentPage = clampPage(page);
  const category = helpCategories[currentPage]!;

  const embed = new EmbedBuilder()
    .setColor(0xd7b4ff)
    .setTitle(`${category.emoji} Help — ${category.name}`)
    .setDescription(
      `${category.description}\n\n${category.commands
        .map((command) => formatCommand(prefix, command))
        .join("\n\n")}`
    )
    .setFooter({ text: `Category ${currentPage + 1}/${totalPages}` });

  return { embeds: [embed] };
}
