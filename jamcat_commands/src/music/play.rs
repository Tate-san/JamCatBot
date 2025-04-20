use super::prelude::*;

#[poise::command(prefix_command, guild_only, aliases("p"), category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[rest]
    #[description = "URL or keywords to search by"]
    query: String,
) -> Result<(), Error> {
    jamcat_music::voice::get_call_or_join(&ctx, false).await?;
    jamcat_music::play_track_url(&ctx, query).await?;
    Ok(())
}
