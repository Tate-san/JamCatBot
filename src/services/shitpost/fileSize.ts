import { stat } from "node:fs/promises";

export async function getFileSize(path: string) {
  return (await stat(path)).size;
}
