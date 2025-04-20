use jamcat_utils::api::waifu::*;
use crate::prelude::*;

#[poise::command(prefix_command, slash_command, category = "NSFW")]
pub async fn hentai(ctx: Context<'_>) -> Result<(), Error> {
    let api = WaifuApi::new()?;
    let image = api.search(true, None).await?;

    ctx.say(image.url).await?;

    Ok(())
}

