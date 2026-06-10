import { AttachmentBuilder, PermissionFlagsBits } from "discord.js";
import { fileURLToPath } from "node:url";
import { defineCommand } from "../types.js";

const jamcatPath = fileURLToPath(
  new URL("../../../assets/images/jamcat.gif", import.meta.url)
);

export const jamCommand = defineCommand({
  name: "jam",
  requiredBotPermissions: [PermissionFlagsBits.AttachFiles],

  async executePrefix({ message }) {
    await message.reply({
      files: [new AttachmentBuilder(jamcatPath, { name: "jamcat.gif" })]
    });
  }
});
