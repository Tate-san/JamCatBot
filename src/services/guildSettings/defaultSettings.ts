export const DEFAULT_GUILD_SETTINGS = {
  prefix: "!",
  language: "en",
  musicVolume: 50,
  idleLeaveEnabled: true,
  idleLeaveSeconds: 300
};

export type GuildSettingsPatch = Partial<typeof DEFAULT_GUILD_SETTINGS>;
