import type { ChatInputCommandInteraction } from 'discord.js';

export class ProgressMessage {
  constructor(private interaction: ChatInputCommandInteraction) {}

  async set(content: string) {
    if (this.interaction.deferred || this.interaction.replied) {
      await this.interaction.editReply({
        content,
      });
    } else {
      await this.interaction.reply({
        content,
      });
    }
  }
}
