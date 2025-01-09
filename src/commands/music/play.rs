use crate::{music, voice};

use super::prelude::*;

#[poise::command(prefix_command, guild_only, aliases("p"), category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[rest]
    #[description = "URL or keywords to search by"]
    query: String,
) -> Result<(), Error> {
    voice::get_call_or_join(&ctx, false).await?;
    music::play_track(&ctx, query).await?;

    Ok(())
}
