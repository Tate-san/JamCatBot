use std::{cmp::Ordering, time::Duration};

use songbird::tracks::{LoopState, TrackState};

use super::prelude::*;
use crate::{
    api,
    music::types::TrackInfo,
    utils::{duration_string, get_total_pages},
};

static TRACK_BAR_SIZE: usize = 15;

pub fn create_coomer_image_embed(
    creator: &api::coomer::types::CreatorInfo,
    image_url: String,
    creator_url: String,
    creator_icon_url: String,
) -> serenity::CreateEmbed {
    serenity::CreateEmbed::new()
        .url(&image_url)
        .image(&image_url)
        .title("Content")
        .author(
            serenity::CreateEmbedAuthor::new(&creator.name)
                .url(creator_url)
                .icon_url(creator_icon_url),
        )
        .field("Service", &creator.service, false)
        .field("Popularity", format!("{}", creator.favorited), false)
        .color(serenity::Colour::MEIBE_PINK)
}

pub fn create_now_playing_embed(info: &TrackInfo, state: &TrackState) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::new()
        .author(serenity::CreateEmbedAuthor::new("🎧 Playing right now"))
        .color(serenity::Colour::MEIBE_PINK)
        .title(info.full_name())
        .url(info.url.clone())
        .thumbnail(info.thumbnail.clone());

    let elapsed = state.position;

    if let Some(duration) = info.duration {
        let duration_secs = duration.as_secs();
        // We wanna clamp it cuz on loop this value gets higher and higher
        let elapsed_secs = elapsed.as_secs() % duration_secs;

        let elapsed_str = duration_string(Duration::from_secs(elapsed_secs));
        let time_str = duration_string(duration);

        let progress = elapsed_secs as f64 / duration_secs as f64;
        let current_bar_progress = (TRACK_BAR_SIZE as f64 * progress).floor() as usize;

        let mut progress_bar_str = String::new();

        for i in 0..TRACK_BAR_SIZE {
            progress_bar_str += match i.cmp(&current_bar_progress) {
                Ordering::Less => "━",
                Ordering::Equal => "⬤",
                Ordering::Greater => "─",
            }
        }

        embed = embed.field(
            "",
            format!("**◁  Ⅱ  ▷    {progress_bar_str}    {elapsed_str} / {time_str}**"),
            false,
        );

        match state.loops {
            LoopState::Infinite => {
                embed = embed.field("", "🔁 Infinite looping".to_string(), false);
            }
            LoopState::Finite(0) => {}
            LoopState::Finite(loops) => {
                embed = embed.field("", format!("🔁 Remaining loops: {loops}"), false);
            }
        }

        embed = embed.field(
            "",
            format!("**{}**", crate::utils::ascii::random_brainrot()),
            false,
        );
    }

    embed
}

pub fn create_queued_track_embed(info: TrackInfo) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::new()
        .author(serenity::CreateEmbedAuthor::new("🚀 Added to queue"))
        .color(serenity::Colour::MEIBE_PINK)
        .title(info.full_name())
        .url(info.url)
        .thumbnail(info.thumbnail);

    if let Some(duration) = info.duration {
        let time_str = duration_string(duration);

        embed = embed.field("", format!("**Duration: {time_str}**"), false);

        embed = embed.field(
            "",
            format!("**{}**", crate::utils::ascii::random_brainrot()),
            false,
        );
    }

    embed
}

pub fn create_queued_tracks_embed(count: usize) -> serenity::CreateEmbed {
    serenity::CreateEmbed::new()
        .author(serenity::CreateEmbedAuthor::new(format!(
            "🚀 Added to queue {count} tracks"
        )))
        .field(
            "",
            format!("**{}**", crate::utils::ascii::random_brainrot()),
            false,
        )
        .color(serenity::Colour::MEIBE_PINK)
}

pub fn create_queue_list_embed(
    mut tracks: Vec<TrackInfo>,
    page: usize,
    items_per_page: usize,
) -> serenity::CreateEmbed {
    // Substracting 1 to exclude the first track
    let tracks_len = tracks.len() - 1;
    let total_pages = get_total_pages(tracks_len, items_per_page).max(1);
    let offset = items_per_page * page;

    let mut embed = serenity::CreateEmbed::new()
        .title(format!("🎧 Tracks Queue ({}/{total_pages})", page + 1))
        .color(serenity::Colour::MEIBE_PINK);

    if let Some(current_track) = tracks.first() {
        embed = embed.field(
            "",
            format!(
                "🔥 **[{}]({})**",
                current_track.full_name(),
                current_track.url
            ),
            false,
        );

        // Remove the first track since its not needed anymore
        tracks.remove(0);
    }

    let from = offset;
    let to = (offset + items_per_page).min(tracks_len);
    let range = from..to;

    for i in range {
        embed = embed.field(
            "",
            format!("{}) [{}]({})", i + 1, tracks[i].full_name(), tracks[i].url),
            false,
        );
    }

    // We dont wanna print out the queue duration when the queue is empty
    // Cuz that would be just all zeroes
    if tracks_len > 0 {
        let mut total_queue_duration_secs = 0;
        for track in tracks {
            if let Some(duration) = track.duration {
                total_queue_duration_secs += duration.as_secs();
            }
        }

        let queue_duration_str = duration_string(Duration::from_secs(total_queue_duration_secs));

        embed = embed.footer(serenity::CreateEmbedFooter::new(format!(
            "Total queue duration: {queue_duration_str}"
        )));
    }

    embed
}
