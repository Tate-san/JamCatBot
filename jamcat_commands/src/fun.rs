use rand::seq::IndexedRandom;

use crate::prelude::*;

#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn yapper(
    ctx: Context<'_>,
    #[description = "Who's the yapper"] user: serenity::User,
) -> Result<(), Error> {
    ctx.send_message(Message::Other(format!(
        "{} is the biggest yapper of em all",
        user.name
    )))
    .await?;

    user.direct_message(
        ctx.http(),
        serenity::CreateMessage::new().content("Quit with the yapping"),
    )
    .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn flip(
    ctx: Context<'_>
) -> Result<(), Error> {
    let coin = vec!["Heads", "Tails"];
    let result = coin.choose(&mut rand::rng());

    if let Some(result) = result {
        let _ = ctx.send_message(Message::Other(format!("🪙 Landed **{result}** 🪙"))).await;
    }
    else {
        return Err(Error::Generic("Unable to choose".to_string()));
    }

    Ok(())
}