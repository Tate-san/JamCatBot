import i18next from "i18next";
import { readFile } from "node:fs/promises";

async function loadLocale(locale: string) {
  return JSON.parse(
    await readFile(new URL(`./locales/${locale}.json`, import.meta.url), "utf8")
  ) as Record<string, unknown>;
}

export async function initI18n() {
  const [en, sassy] = await Promise.all([loadLocale("en"), loadLocale("sassy")]);

  await i18next.init({
    lng: "en",
    fallbackLng: "en",
    resources: {
      en: {
        translation: en
      },
      sassy: {
        translation: sassy
      }
    },
    interpolation: {
      escapeValue: false,
      prefix: "{",
      suffix: "}"
    }
  });
}

export { i18next };
