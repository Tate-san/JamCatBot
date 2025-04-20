use std::{str::FromStr, time::Duration};

use crate::{prelude::*, providers::{spotify::SPOTIFY, ytdlp::Ytdl}};
use jamcat_common::types::music::*;
use jamcat_core::prelude::*;
use url::Url;
use crate::constants;


/// Main function for playing/enqueuing tracks.
pub async fn play_track_url(ctx: &Context<'_>, query: String) -> Result<(), BotError> {
    let query_type = match_query(query).await?;

    let queue = ctx.get_queue().await?;
    let current_handle = queue.lock().await.current_track_handle();

    match query_type {
        QueryType::TrackLink(url) => {
            let track_info = enqueue_back(ctx, url).await?;

            if current_handle.is_some() {
                ctx.send_embed(message::create_queued_track_embed(track_info))
                    .await?;
            }
        },
        QueryType::PlaylistLink(url) => {
            let ytdl = Ytdl::new();
            let playlist = ytdl.query_playlist(&url).await?;
            let playlist_len = playlist.len();

            let message = ctx
                .send_message(Message::Other("Fetching songs".to_string()))
                .await?;

            for (index, item) in playlist.iter().enumerate() {
                message
                    .edit(
                        *ctx,
                        poise::CreateReply::default().embed(
                            Message::Other(format!("Adding song {}/{playlist_len}", index + 1))
                                .into(),
                        ),
                    )
                    .await?;

                if let Err(error) = enqueue_back(ctx, item.url.clone()).await {
                    ctx.send_message(Message::Error(error.to_string())).await?;

                    match error {
                        BotError::MusicError(MusicError::Unavailable(_)) => continue,
                        _ => return Err(error),
                    };
                }
            }

            message
                .edit(
                    *ctx,
                    poise::CreateReply::default()
                        .embed(message::create_queued_tracks_embed(playlist_len)),
                )
                .await?;
        },
        QueryType::Keywords(query) => {
            let track = Ytdl::new().search_song(&query).await?;
            let track_info = enqueue_back(ctx, track.url).await?;

            if current_handle.is_some() {
                ctx.send_embed(message::create_queued_track_embed(track_info))
                    .await?;
            }
        },
        QueryType::KeywordsList(list) => {
            let ytdl = Ytdl::new();
            let list_len = list.len();


            let message = ctx
                .send_message(Message::Other("Fetching songs".to_string()))
                .await?;

            for (index, keyword) in list.iter().enumerate() {
                message
                    .edit(
                        *ctx,
                        poise::CreateReply::default().embed(
                            Message::Other(format!("Adding song {}/{list_len}", index + 1)).into(),
                        ),
                    )
                    .await?;

                let track = ytdl.search_song(keyword).await?;

                if let Err(error) = enqueue_back(ctx, track.url).await {
                    ctx.send_message(Message::Error(error.to_string())).await?;

                    match error {
                        BotError::MusicError(MusicError::Unavailable(_)) => continue,
                        _ => return Err(error),
                    };
                }
            }

            message
                .edit(
                    *ctx,
                    poise::CreateReply::default()
                        .embed(message::create_queued_tracks_embed(list_len)),
                )
                .await?;
        }
    }

    Ok(())
}

/// Matches the query string to corresponding QueryType.
///
/// Also handles extraction from ytdl unsupported sites like spotify.
async fn match_query(query: String) -> Result<QueryType, BotError> {
    Ok(match Url::from_str(&query) {
        Ok(url) => match url.domain() {
            Some("open.spotify.com") => SPOTIFY.lock().await.extract(url).await?,
            Some(_) => {
                if constants::YT_PLAYLIST_URL_REGEX.is_match(&query) {
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

async fn enqueue_back(ctx: &Context<'_>, url: String) -> Result<TrackInfo, Error> {
    let call = ctx
        .get_bot_call()
        .await
        .map_err(|_| BotError::BotNotInVoice)?;

    let metadata = Ytdl::new().query(&url).await?;


    let track_info = TrackInfo {
        url,
        title: metadata.title.clone().unwrap_or("Unknown".to_string()),
        artist: metadata.author.unwrap_or("Unknown".to_string()),
        thumbnail: metadata.thumbnail.clone().unwrap_or_default(),
        duration: Some(Duration::from_secs_f32(metadata.duration.unwrap_or_default())),
    };

    let queue = ctx.get_queue().await?;
    let mut queue_handle = queue.lock().await;

    queue_handle.enqueue(track_info.clone());

    if queue_handle.current_track_handle().is_none() {
        queue_handle.play_next(call).await?;
    }

    Ok(track_info)
}

pub async fn set_volume(ctx: &Context<'_>, volume: f32) -> Result<(), BotError> {
    let guild_id = ctx.guild_id().ok_or(BotError::GuildOnly)?;

    if let Some(cache) = ctx.data().guild_cache.lock().await.get_mut(&guild_id) {
        cache.queue.lock().await.set_volume(volume)?;
    } else {
        // This edgecase shouldn't happen at all since the cache gets created on guild register event
        tracing::error!("Guild {} is not cached", guild_id);
    }

    let queue = ctx.get_queue().await?;
    queue.lock().await.set_volume(volume)?;

    Ok(())
}