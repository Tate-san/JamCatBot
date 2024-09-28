use super::prelude::*;
use crate::{api, music::types::TrackInfo, utils::duration_string};

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
        .color(serenity::Colour::MEIBE_PINK)
        .title(info.full_name())
        .url(info.url)
        .thumbnail(info.thumbnail);

    if let Some(duration) = info.duration {
        let time_str = duration_string(duration);

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
    tracks: Vec<TrackInfo>,
    page: usize,
    items_per_page: usize,
) -> serenity::CreateEmbed {
    let tracks_len = tracks.len();
    let offset = items_per_page * page;

    let mut embed = serenity::CreateEmbed::new()
        .title("🎧 Tracks Queue")
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
    }

    for i in (offset + 1)..(offset + items_per_page + 1) {
        if i < tracks_len {
            embed = embed.field(
                "",
                format!("{}) [{}]({})", i, tracks[i].full_name(), tracks[i].url),
                false,
            );
        }
    }

    embed
}
