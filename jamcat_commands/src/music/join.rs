use super::prelude::*;

#[poise::command(prefix_command, guild_only, category = "Voice")]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    jamcat_music::voice::get_call_or_join(&ctx, true).await?;
    Ok(())
}
