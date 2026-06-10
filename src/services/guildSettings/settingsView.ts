import { EmbedBuilder } from "discord.js";
import { t } from "../../i18n/t.js";

const PAGE_SIZE = 10;

type SettingsViewData = {
  guildId?: string;
  prefix: string;
  language: string;
  musicVolume: number;
  idleLeaveEnabled: boolean;
  idleLeaveSeconds: number;
  createdAt?: Date | string;
  updatedAt?: Date | string;
};

function clampPage(page: number, totalPages: number) {
  return Math.min(Math.max(page, 0), Math.max(totalPages - 1, 0));
}

function formatValue(value: unknown) {
  if (value instanceof Date) return value.toISOString();
  if (typeof value === "boolean") return value ? "enabled" : "disabled";
  return String(value);
}

function getSettingsItems(settings: SettingsViewData) {
  const items: Array<[string, unknown]> = [
    ["Prefix", settings.prefix],
    ["Language", settings.language],
    ["Music volume", `${settings.musicVolume}%`],
    ["Idle leave", settings.idleLeaveEnabled],
    ["Idle leave seconds", settings.idleLeaveSeconds],
    ["Guild ID", settings.guildId],
    ["Created at", settings.createdAt],
    ["Updated at", settings.updatedAt]
  ];

  return items.filter((item): item is [string, {}] => item[1] !== undefined);
}

export function getSettingsPageCount(settings: SettingsViewData) {
  return Math.max(Math.ceil(getSettingsItems(settings).length / PAGE_SIZE), 1);
}

export function buildSettingsPage(
  settings: SettingsViewData,
  language: string,
  page = 0,
  totalPages = getSettingsPageCount(settings)
) {
  const items = getSettingsItems(settings);
  const currentPage = clampPage(page, totalPages);
  const pageStart = currentPage * PAGE_SIZE;
  const pageItems = items.slice(pageStart, pageStart + PAGE_SIZE);
  const description = pageItems
    .map(([label, value]) => `- ${label}: \`${formatValue(value)}\``)
    .join("\n");

  const embed = new EmbedBuilder()
    .setColor(0x9bdb8b)
    .setTitle(t(language, "settings.show.title"))
    .setDescription(description || t(language, "settings.show.empty"))
    .setFooter({
      text: t(language, "settings.show.page", {
        page: currentPage + 1,
        pages: totalPages
      })
    });

  return { embeds: [embed] };
}
