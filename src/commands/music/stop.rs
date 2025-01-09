use super::prelude::*;
use crate::voice;

#[poise::command(prefix_command, guild_only, category = "Music")]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    voice::leave_call(&ctx, false).await
}
