pub mod error;
pub mod types;

use crate::messages;
use crate::prelude::*;
use crate::sources::spotify::SPOTIFY;
use crate::sources::ytdl::Ytdl;
use crate::types::*;
use error::MusicError;
use lazy_static::lazy_static;
use regex::Regex;
use songbird::input::Compose;
use songbird::input::YoutubeDl;
use songbird::tracks::TrackHandle;
use std::str::FromStr;
use types::QueryType;
use types::TrackInfo;
use url::Url;

lazy_static! {
    // Regex for keywords that indicate the url is a playlist
    pub static ref PLAYLIST_URL_REGEX: Regex = Regex::new(r"list=").unwrap();
}

/// Main function for playing tracks.
pub async fn play_track(ctx: &Context<'_>, query: String) -> Result<(), Error> {
    let call = ctx.get_bot_call().await?;
    let query_type = match_query(query).await?;

    let queue_len = call.lock().await.queue().len();

    match query_type {
        QueryType::TrackLink(url) => {
            let (_, track_info) = enqueue_back(ctx, url).await?;

            if queue_len >= 1 {
                ctx.send_embed(messages::factory::create_queued_track_embed(track_info))
                    .await?;
            }
        }
        QueryType::PlaylistLink(url) => {
            let ytdl = Ytdl::new();
            let playlist = ytdl.query_playlist(&url).await?;
            let playlist_len = playlist.len();

            for item in playlist {
                let _ = enqueue_back(ctx, item.url).await?;
            }

            ctx.send_embed(messages::factory::create_queued_tracks_embed(playlist_len))
                .await?;
        }
        QueryType::Keywords(query) => {
            let ytdl = Ytdl::new();
            let track = ytdl.search_song(&query).await?;

            let (_, track_info) = enqueue_back(ctx, track.url.clone()).await?;

            if queue_len >= 1 {
                ctx.send_embed(messages::factory::create_queued_track_embed(track_info))
                    .await?;
            }
        }
        QueryType::KeywordsList(list) => {
            let ytdl = Ytdl::new();
            let list_len = list.len();

            for keyword in list {
                let track = ytdl.search_song(&keyword).await?;
                let (_, _) = enqueue_back(ctx, track.url.clone()).await?;
            }

            ctx.send_embed(messages::factory::create_queued_tracks_embed(list_len))
                .await?;
        }
        _ => {
            return Err(BotError::Generic(
                "This shit is not implemented yet or won't be supported at all".to_string(),
            ))
        }
    };

    Ok(())
}

/// Matches the query string to corresponding QueryType.
///
/// Also handles extraction from ytdl unsupported sites like spotify.
async fn match_query(query: String) -> Result<QueryType, Error> {
    Ok(match Url::from_str(&query) {
        Ok(url) => match url.domain() {
            Some("open.spotify.com") => SPOTIFY.lock().await.extract(url).await?,
            Some(_) => {
                if PLAYLIST_URL_REGEX.is_match(&query) {
                    QueryType::PlaylistLink(query)
                } else {
                    QueryType::TrackLink(query)
                }
            }
            None => return Err(MusicError::InvalidLink.into()),
        },

        Err(_) => QueryType::Keywords(query),
    })
}

async fn enqueue_back(ctx: &Context<'_>, url: String) -> Result<(TrackHandle, TrackInfo), Error> {
    let guild_id = ctx.guild_id().ok_or(BotError::GuildOnly)?;

    let call = ctx
        .get_bot_call()
        .await
        .map_err(|_| BotError::BotNotInVoice)?;

    let mut source = YoutubeDl::new(ctx.data().http.clone(), url.clone());

    let metadata = source
        .aux_metadata()
        .await
        .map_err(|_| MusicError::TrackFetch)?;

    let mut handler = call.lock().await;
    let track_handle = handler.enqueue(source.into()).await;
    let mut typemap = track_handle.typemap().write().await;

    let track_info = TrackInfo {
        url,
        title: metadata.title.clone().unwrap_or("Unknown".to_string()),
        artist: metadata.artist.unwrap_or("Unknown".to_string()),
        thumbnail: metadata.thumbnail.clone().unwrap_or_default(),
        duration: metadata.duration.clone(),
    };

    typemap.insert::<TrackInfo>(track_info.clone());

    // Drop the borrow so we can return the handle
    drop(typemap);

    if let Some(cache) = ctx.data().guild_cache.lock().await.get_mut(&guild_id) {
        let _ = track_handle.set_volume(cache.volume / 100.0);
    }

    Ok((track_handle, track_info))
}

pub async fn set_volume(ctx: &Context<'_>, volume: f32) -> Result<(), Error> {
    let guild_id = ctx.guild_id().ok_or(BotError::GuildOnly)?;
    let call = ctx.get_bot_call().await?;

    if let Some(cache) = ctx.data().guild_cache.lock().await.get_mut(&guild_id) {
        cache.volume = volume;
    } else {
        // This edgecase shouldn't happen at all since the cache gets created on guild register event
        tracing::error!("Guild {} is not cached", guild_id);
    }

    let handle = call.lock().await;

    handle.queue().modify_queue(|queue| {
        for track in queue {
            let _ = track.set_volume(volume / 100.0);
        }
    });

    Ok(())
}
