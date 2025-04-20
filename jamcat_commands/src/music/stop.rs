use super::prelude::*;

#[poise::command(prefix_command, guild_only, category = "Music")]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    jamcat_music::voice::leave_call(&ctx, false).await
}
