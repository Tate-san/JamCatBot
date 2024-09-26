use crate::{downloaders::ytdl::Ytdl, music, voice};

use super::prelude::*;

#[poise::command(prefix_command, guild_only, aliases("p"), category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[rest]
    #[description = "URL or keywords to search by"]
    query: String,
) -> Result<(), Error> {
    if let Err(error) = voice::get_call_or_join(&ctx).await {
        ctx.send_message(Message::Error(error.to_string())).await?;
        return Ok(());
    };

    if let Err(error) = music::play_url(&ctx, query).await {
        ctx.send_message(Message::Error(error.to_string())).await?;
        return Ok(());
    }

    Ok(())
}
