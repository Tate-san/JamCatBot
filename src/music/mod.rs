pub mod types;

use crate::downloaders::ytdl::Ytdl;
use crate::messages;
use crate::prelude::*;
use crate::types::*;
use anyhow::Result;
use songbird::input::Compose;
use types::QueryType;
use types::TrackInfo;

pub async fn play_url(ctx: &Context<'_>, query: String) -> Result<()> {
    let call = ctx.get_bot_call().await?;
    let query_type = QueryType::from_url(query);

    let mut url_list: Vec<String> = vec![];

    match query_type {
        QueryType::Other(url) | QueryType::Youtube(url) => {
            url_list.push(url);
        }
        QueryType::YoutubePlaylist(url) => {
            let ytdl = Ytdl::new();
            let playlist = ytdl.get_playlist_items(url).await?;

            for item in playlist {
                url_list.push(item.url);
            }
        }
        _ => {
            return Err(anyhow::anyhow!("I don't know what to do with this shit"));
        }
    }

    let mut handler = call.lock().await;

    let initial_queue_len = handler.queue().len();
    let url_list_len = url_list.len();
    let mut first_track = None;

    for url in url_list {
        let mut src = songbird::input::YoutubeDl::new(ctx.data().http.clone(), url.clone());

        let metadata = match src.aux_metadata().await {
            Ok(res) => res,
            Err(_) => {
                return Err(anyhow::anyhow!("Unable to fetch metadata"));
            }
        };

        let track_handle = handler.enqueue(src.into()).await;
        let mut typemap = track_handle.typemap().write().await;

        let track_info = TrackInfo {
            url: url.clone(),
            title: metadata.title.clone().unwrap_or("Unknown".to_string()),
            thumbnail: metadata.thumbnail.clone().unwrap_or_default(),
            duration: metadata.duration.clone(),
        };

        if first_track.is_none() {
            first_track = Some(track_info.clone());
        }

        typemap.insert::<TrackInfo>(track_info);
    }

    if initial_queue_len > 0 {
        if url_list_len == 1 {
            let track_info = first_track.unwrap();
            ctx.send_embed(messages::factory::create_queued_track_embed(track_info))
                .await?;
        } else {
            ctx.send_embed(messages::factory::create_queued_tracks_embed(url_list_len))
                .await?;
        }
    }

    anyhow::Ok(())
}
