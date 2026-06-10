import {
  ActionRowBuilder,
  ButtonBuilder,
  ButtonStyle,
  StringSelectMenuBuilder,
  type Message,
  type MessageCreateOptions
} from "discord.js";
import { t } from "../../i18n/t.js";

export type PaginationStyle = "buttons" | "dropdown" | "buttons-dropdown";

export type PaginationPage = Pick<
  MessageCreateOptions,
  "allowedMentions" | "embeds" | "files"
> & {
  content?: string;
};

type PaginationRow =
  | ActionRowBuilder<ButtonBuilder>
  | ActionRowBuilder<StringSelectMenuBuilder>;

export type PaginationPayload = PaginationPage & {
  components: PaginationRow[];
};

type PaginationContext = {
  page: number;
  totalPages: number;
  disabled: boolean;
};

type SendPaginatedMessageOptions = {
  ownerId: string;
  language: string;
  style?: PaginationStyle;
  initialPage?: number;
  timeoutMs?: number;
  getTotalPages(): number | Promise<number>;
  getPageLabel?(page: number, totalPages: number): string;
  buildPage(ctx: PaginationContext): PaginationPage | Promise<PaginationPage>;
  send(payload: PaginationPayload): Promise<Message>;
};

const DEFAULT_TIMEOUT_MS = 5 * 60 * 1000;
const PREVIOUS_BUTTON_ID = "pagination:previous";
const NEXT_BUTTON_ID = "pagination:next";
const SELECT_MENU_ID = "pagination:select";
const MAX_SELECT_OPTIONS = 25;

function clampPage(page: number, totalPages: number) {
  return Math.min(Math.max(page, 0), Math.max(totalPages - 1, 0));
}

function getSelectWindow(page: number, totalPages: number) {
  const size = Math.min(totalPages, MAX_SELECT_OPTIONS);
  const half = Math.floor(size / 2);
  const start = Math.min(Math.max(page - half, 0), totalPages - size);

  return Array.from({ length: size }, (_, index) => start + index);
}

function buildPaginationComponents(options: {
  page: number;
  totalPages: number;
  style: PaginationStyle;
  disabled: boolean;
  getPageLabel?: (page: number, totalPages: number) => string;
}) {
  if (options.totalPages <= 1) return [];

  const rows: PaginationRow[] = [];
  const includeButtons =
    options.style === "buttons" || options.style === "buttons-dropdown";
  const includeDropdown =
    options.style === "dropdown" || options.style === "buttons-dropdown";

  if (includeButtons) {
    rows.push(
      new ActionRowBuilder<ButtonBuilder>().addComponents(
        new ButtonBuilder()
          .setCustomId(PREVIOUS_BUTTON_ID)
          .setEmoji("⬅️")
          .setLabel("Previous")
          .setStyle(ButtonStyle.Secondary)
          .setDisabled(options.disabled || options.page <= 0),
        new ButtonBuilder()
          .setCustomId(NEXT_BUTTON_ID)
          .setEmoji("➡️")
          .setLabel("Next")
          .setStyle(ButtonStyle.Secondary)
          .setDisabled(options.disabled || options.page >= options.totalPages - 1)
      )
    );
  }

  if (includeDropdown) {
    rows.push(
      new ActionRowBuilder<StringSelectMenuBuilder>().addComponents(
        new StringSelectMenuBuilder()
          .setCustomId(SELECT_MENU_ID)
          .setPlaceholder(`Page ${options.page + 1}/${options.totalPages}`)
          .setDisabled(options.disabled)
          .addOptions(
            getSelectWindow(options.page, options.totalPages).map((page) => ({
              label:
                options.getPageLabel?.(page, options.totalPages).slice(0, 100) ??
                `Page ${page + 1}`,
              value: String(page),
              default: page === options.page
            }))
          )
      )
    );
  }

  return rows;
}

async function buildPaginationPayload(options: {
  page: number;
  disabled: boolean;
  style: PaginationStyle;
  getTotalPages(): number | Promise<number>;
  getPageLabel?: (page: number, totalPages: number) => string;
  buildPage(ctx: PaginationContext): PaginationPage | Promise<PaginationPage>;
}) {
  const totalPages = Math.max(1, await options.getTotalPages());
  const page = clampPage(options.page, totalPages);
  const pagePayload = await options.buildPage({
    page,
    totalPages,
    disabled: options.disabled
  });

  return {
    page,
    totalPages,
    payload: {
      ...pagePayload,
      components: buildPaginationComponents({
        page,
        totalPages,
        style: options.style,
        disabled: options.disabled,
        getPageLabel: options.getPageLabel
      })
    }
  };
}

export async function sendPaginatedMessage(options: SendPaginatedMessageOptions) {
  const style = options.style ?? "buttons";
  let page = options.initialPage ?? 0;
  const initial = await buildPaginationPayload({
    page,
    disabled: false,
    style,
    getTotalPages: options.getTotalPages,
    getPageLabel: options.getPageLabel,
    buildPage: options.buildPage
  });

  page = initial.page;
  const message = await options.send(initial.payload);

  if (initial.totalPages <= 1) return message;

  const collector = message.createMessageComponentCollector({
    time: options.timeoutMs ?? DEFAULT_TIMEOUT_MS
  });

  collector.on("collect", async (component) => {
    if (component.user.id !== options.ownerId) {
      await component.reply({
        content: t(options.language, "common.notYourControl"),
        ephemeral: true
      });
      return;
    }

    if (component.isButton()) {
      if (![PREVIOUS_BUTTON_ID, NEXT_BUTTON_ID].includes(component.customId)) {
        return;
      }

      page += component.customId === NEXT_BUTTON_ID ? 1 : -1;
    } else if (component.isStringSelectMenu()) {
      if (component.customId !== SELECT_MENU_ID) return;
      page = Number(component.values[0]);
    } else {
      return;
    }

    const next = await buildPaginationPayload({
      page,
      disabled: false,
      style,
      getTotalPages: options.getTotalPages,
      getPageLabel: options.getPageLabel,
      buildPage: options.buildPage
    });

    page = next.page;
    await component.update(next.payload);
  });

  collector.on("end", async () => {
    const expired = await buildPaginationPayload({
      page,
      disabled: true,
      style,
      getTotalPages: options.getTotalPages,
      getPageLabel: options.getPageLabel,
      buildPage: options.buildPage
    }).catch(() => undefined);

    if (!expired) return;
    await message.edit(expired.payload).catch(() => undefined);
  });

  return message;
}
