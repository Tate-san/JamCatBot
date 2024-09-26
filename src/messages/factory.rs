use super::prelude::*;
use crate::{api, music::types::TrackInfo};

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

pub fn create_now_playing_embed(info: TrackInfo) -> serenity::CreateEmbed {
    let mut embed = serenity::CreateEmbed::new()
        .author(serenity::CreateEmbedAuthor::new("🎧 Playing right now"))
        .title(info.title)
        .url(info.url)
        .thumbnail(info.thumbnail);

    if let Some(duration) = info.duration {
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / 60) / 60;

        let time_str = if hours > 0 {
            format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
        } else {
            format!("{minutes:0>2}:{seconds:0>2}")
        };

        embed = embed.field(
            "",
            format!("**◁  Ⅱ  ▷    ━━⬤───────    00:00 / {time_str}**"),
            false,
        );

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
        .title(info.title)
        .url(info.url)
        .thumbnail(info.thumbnail);

    if let Some(duration) = info.duration {
        let seconds = duration.as_secs() % 60;
        let minutes = (duration.as_secs() / 60) % 60;
        let hours = (duration.as_secs() / 60) / 60;

        let time_str = if hours > 0 {
            format!("{hours:0>2}:{minutes:0>2}:{seconds:0>2}")
        } else {
            format!("{minutes:0>2}:{seconds:0>2}")
        };

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
}
