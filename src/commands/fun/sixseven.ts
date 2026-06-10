import { AttachmentBuilder, PermissionFlagsBits } from "discord.js";
import { fileURLToPath } from "node:url";
import { defineCommand } from "../types.js";

const sixSevenCatPath = fileURLToPath(
  new URL("../../../assets/images/cat-67.gif", import.meta.url)
);

export const sixSevenCommand = defineCommand({
  name: "sixseven",
  aliases: ["67"],
  requiredBotPermissions: [PermissionFlagsBits.AttachFiles],

  async executePrefix({ message }) {
    await message.reply({
      files: [new AttachmentBuilder(sixSevenCatPath, { name: "cat-67.gif" })]
    });
  }
});
