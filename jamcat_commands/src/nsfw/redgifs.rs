use jamcat_utils::api::redgifs::*;
use crate::prelude::*;

#[poise::command(prefix_command, slash_command, category = "NSFW")]
pub async fn naughty(ctx: Context<'_>) -> Result<(), Error> {
    let mut redgifs = RedgifsApi::new()?;
    redgifs.login_temporary().await?;
    let res = redgifs.feed().await;

    match res {
        Ok(list) => {
            let gif_count = list.gifs.len();
            let random_gif: usize = rand::rng().random_range(0..gif_count);

            let link = list.gifs[random_gif].urls.hd.clone().unwrap_or_default();

            ctx.say(link).await?;
        }
        Err(error) => {
            return Err(BotError::Generic(error.to_string()));
        }
    }

    Ok(())
}