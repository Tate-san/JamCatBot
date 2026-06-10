import ffmpegStatic from "ffmpeg-static";
import { execa } from "execa";
import { basename, extname, join } from "node:path";
import { env } from "../../config/env.js";
import { getFileSize } from "./fileSize.js";

function ffmpegPath() {
  return env.FFMPEG_PATH ?? ffmpegStatic ?? "ffmpeg";
}

async function getDurationSeconds(path: string) {
  try {
    await execa(ffmpegPath(), ["-i", path], { reject: true });
  } catch (error) {
    const stderr = error instanceof Error && "stderr" in error ? String(error.stderr) : "";
    const match = /Duration: (\d+):(\d+):(\d+(?:\.\d+)?)/.exec(stderr);

    if (!match) return undefined;

    return Number(match[1]) * 3600 + Number(match[2]) * 60 + Number(match[3]);
  }

  return undefined;
}

export async function compressMediaToLimit(inputPath: string, maxBytes: number) {
  const extension = extname(inputPath) || ".mp4";
  const outputPath = join(
    inputPath.slice(0, -extension.length) + `-compressed${extension}`
  );
  const duration = await getDurationSeconds(inputPath);
  const audioKbps = 64;
  const targetBytes = Math.floor(maxBytes * 0.92);
  let videoKbps = duration
    ? Math.max(120, Math.floor((targetBytes * 8) / duration / 1000 - audioKbps))
    : 500;

  let latestOutput = outputPath;

  for (let attempt = 0; attempt < 3; attempt += 1) {
    latestOutput = outputPath.replace(extension, `-${attempt + 1}${extension}`);

    await execa(
      ffmpegPath(),
      [
        "-y",
        "-i",
        inputPath,
        "-c:v",
        "libx264",
        "-preset",
        "veryfast",
        "-b:v",
        `${videoKbps}k`,
        "-maxrate",
        `${videoKbps}k`,
        "-bufsize",
        `${videoKbps * 2}k`,
        "-c:a",
        "aac",
        "-b:a",
        `${audioKbps}k`,
        "-movflags",
        "+faststart",
        latestOutput
      ],
      { stderr: "pipe", stdout: "pipe" }
    );

    if ((await getFileSize(latestOutput)) <= maxBytes) {
      return {
        path: latestOutput,
        filename: basename(latestOutput)
      };
    }

    videoKbps = Math.max(80, Math.floor(videoKbps * 0.75));
  }

  // Final hard cap fallback. It can truncate awkwardly, but preserves the 10MB contract.
  const cappedOutput = outputPath.replace(extension, `-capped${extension}`);
  await execa(
    ffmpegPath(),
    [
      "-y",
      "-i",
      inputPath,
      "-fs",
      String(maxBytes),
      "-c:v",
      "libx264",
      "-preset",
      "veryfast",
      "-c:a",
      "aac",
      cappedOutput
    ],
    { stderr: "pipe", stdout: "pipe" }
  );

  return {
    path: cappedOutput,
    filename: basename(cappedOutput)
  };
}
