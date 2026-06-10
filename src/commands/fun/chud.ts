import { AttachmentBuilder, PermissionFlagsBits } from "discord.js";
import { fileURLToPath } from "node:url";
import { defineCommand } from "../types.js";

const chudPath = fileURLToPath(
  new URL("../../../assets/images/chud.gif", import.meta.url)
);

export const chudCommand = defineCommand({
  name: "chud",
  requiredBotPermissions: [PermissionFlagsBits.AttachFiles],

  async executePrefix({ message }) {
    await message.reply({
      files: [new AttachmentBuilder(chudPath, { name: "chud.gif" })]
    });
  }
});
