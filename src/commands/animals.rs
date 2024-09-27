use super::prelude::*;
use crate::api;

#[poise::command(prefix_command, slash_command, category = "Animals")]
pub async fn cat(ctx: Context<'_>) -> Result<(), Error> {
    let api = api::CatsApi::new()?;
    let res = api.random_cat().await?;

    ctx.send_message(Message::Image(res.url)).await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, category = "Animals")]
pub async fn dog(ctx: Context<'_>) -> Result<(), Error> {
    let api = api::DogsApi::new()?;
    let res = api.random_cat().await?;

    ctx.send_message(Message::Image(res.url)).await?;

    Ok(())
}
