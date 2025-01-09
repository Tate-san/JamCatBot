use super::prelude::*;
use crate::voice;

#[poise::command(prefix_command, guild_only, category = "Voice")]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    voice::leave_call(&ctx, true).await
}
