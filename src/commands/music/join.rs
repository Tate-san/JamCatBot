use super::prelude::*;
use crate::voice;

#[poise::command(prefix_command, guild_only, category = "Voice")]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    voice::get_call_or_join(&ctx, true).await?;
    Ok(())
}
