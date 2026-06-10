import type { Client, ClientEvents } from "discord.js";
import { events } from "../events/index.js";

export type BotEvent<K extends keyof ClientEvents = keyof ClientEvents> = {
  name: K;
  once?: boolean;
  execute(...args: ClientEvents[K]): Promise<void> | void;
};

export function registerEvents(client: Client) {
  for (const event of events) {
    const execute = (...args: unknown[]) =>
      void (event.execute as (...args: unknown[]) => Promise<void> | void)(...args);

    if (event.once) {
      client.once(event.name, execute);
    } else {
      client.on(event.name, execute);
    }
  }
}
