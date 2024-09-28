use crate::{music, voice};

use super::prelude::*;

#[poise::command(prefix_command, guild_only, aliases("p"), category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[rest]
    #[description = "URL or keywords to search by"]
    query: String,
) -> Result<(), Error> {
    if let Err(error) = voice::get_call_or_join(&ctx).await {
        return Err(error.into());
    };

    if let Err(error) = music::play_track(&ctx, query).await {
        return Err(error.into());
    }

    Ok(())
}
