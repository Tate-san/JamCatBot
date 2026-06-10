import { readdir } from "node:fs/promises";
import { basename, join } from "node:path";
import { execa } from "execa";
import { dir } from "tmp-promise";
import { env } from "../../config/env.js";

export type DownloadedMedia = {
  path: string;
  filename: string;
  cleanup(): Promise<void>;
};

export async function downloadMedia(url: string): Promise<DownloadedMedia> {
  const tempDir = await dir({ unsafeCleanup: true });
  const outputTemplate = join(tempDir.path, "media.%(ext)s");

  try {
    await execa(
      env.YTDLP_PATH,
      [
        "--no-playlist",
        "-f",
        "bv*+ba/best",
        "--merge-output-format",
        "mp4",
        "-o",
        outputTemplate,
        url
      ],
      { stderr: "pipe", stdout: "pipe" }
    );

    const files = (await readdir(tempDir.path)).filter(
      (file) => !file.endsWith(".part") && !file.endsWith(".ytdl")
    );
    const file = files[0];

    if (!file) {
      throw new Error("yt-dlp did not produce a media file.");
    }

    const path = join(tempDir.path, file);
    return {
      path,
      filename: basename(path),
      cleanup: tempDir.cleanup
    };
  } catch (error) {
    await tempDir.cleanup();
    throw error;
  }
}
