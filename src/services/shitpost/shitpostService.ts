import { AttachmentBuilder } from "discord.js";
import { DISCORD_UPLOAD_LIMIT_BYTES } from "../../config/constants.js";
import { BotError } from "../commandUX/errors.js";
import { compressMediaToLimit } from "./compressMedia.js";
import { downloadMedia, type DownloadedMedia } from "./downloadMedia.js";
import { getFileSize } from "./fileSize.js";

export type ShitpostProgress = "downloading" | "checkingSize" | "compressing";

export type ProcessedShitpost = {
  attachment: AttachmentBuilder;
  cleanup(): Promise<void>;
};

export class ShitpostService {
  async process(
    url: string,
    onProgress?: (progress: ShitpostProgress) => Promise<void>
  ): Promise<ProcessedShitpost> {
    let downloaded: DownloadedMedia | undefined;

    try {
      await onProgress?.("downloading");
      downloaded = await downloadMedia(url);

      await onProgress?.("checkingSize");
      const size = await getFileSize(downloaded.path);
      const result =
        size <= DISCORD_UPLOAD_LIMIT_BYTES
          ? { path: downloaded.path, filename: downloaded.filename }
          : await (async () => {
              await onProgress?.("compressing");
              return compressMediaToLimit(
                downloaded.path,
                DISCORD_UPLOAD_LIMIT_BYTES
              );
            })();

      return {
        attachment: new AttachmentBuilder(result.path, { name: result.filename }),
        cleanup: downloaded.cleanup
      };
    } catch (error) {
      await downloaded?.cleanup().catch(() => undefined);
      throw new BotError(
        "SHITPOST_FAILED",
        error instanceof Error ? error.message : undefined
      );
    }
  }
}

export const shitpostService = new ShitpostService();
