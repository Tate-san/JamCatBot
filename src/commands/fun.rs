use super::prelude::*;

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
