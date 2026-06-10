export class BotError extends Error {
  constructor(
    public code: string,
    message?: string
  ) {
    super(message ?? code);
    this.name = "BotError";
  }
}

export function toBotErrorCode(error: unknown) {
  return error instanceof BotError ? error.code : undefined;
}
