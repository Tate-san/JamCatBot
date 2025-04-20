use super::prelude::*;

#[poise::command(prefix_command, guild_only, category = "Voice")]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    jamcat_music::voice::leave_call(&ctx, true).await
}
