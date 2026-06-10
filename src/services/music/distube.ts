import { DirectLinkPlugin } from "@distube/direct-link";
import { SpotifyPlugin } from "@distube/spotify";
import { YouTubePlugin } from "@distube/youtube";
import type { Client } from "discord.js";
import { DisTube } from "distube";
import { env } from "../../config/env.js";
import { YtDlpExtractorPlugin } from "./ytDlpExtractorPlugin.js";

export function createDisTube(client: Client) {
  const spotifyApi =
    env.SPOTIFY_CLIENT_ID && env.SPOTIFY_CLIENT_SECRET
      ? {
          clientId: env.SPOTIFY_CLIENT_ID,
          clientSecret: env.SPOTIFY_CLIENT_SECRET,
          topTracksCountry: env.SPOTIFY_TOP_TRACKS_COUNTRY
        }
      : undefined;

  return new DisTube(client, {
    emitNewSongOnly: false,
    savePreviousSongs: true,
    plugins: [
      new SpotifyPlugin({ api: spotifyApi }),
      new DirectLinkPlugin(),
      new YtDlpExtractorPlugin(),
      new YouTubePlugin()
    ]
  });
}
