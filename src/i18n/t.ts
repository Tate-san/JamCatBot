import { i18next } from "./i18n.js";

export function t(language: string, key: string, vars?: Record<string, unknown>) {
  return i18next.t(key, {
    lng: language,
    ...vars
  });
}
