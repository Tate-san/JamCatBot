use super::prelude::*;

#[poise::command(prefix_command, guild_only, category = "Music")]
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id.clone();

    let manager = &ctx.data().songbird;
    let call = ctx.get_bot_call().await;
    let has_handler = call.is_ok();

    if has_handler {
        let user_voice_state = if let Some(voice_state) = ctx.get_author_voice_state().await {
            voice_state
        } else {
            ctx.send_message(Message::Error(
                "You have to be in a voice channel".to_string(),
            ))
            .await?;
            return Ok(());
        };

        let call = call?;

        let call_lock = call.lock().await;

        let user_channel = user_voice_state.channel_id.unwrap();
        let bot_channel = call_lock.current_channel().unwrap();

        if user_channel.get() != bot_channel.0.get() {
            ctx.send_message(Message::Error(
                "You are not in the same voice channel as bot".to_string(),
            ))
            .await?;
            return Ok(());
        }

        // Has to be dropped otherwise the remove causes deadlock
        drop(call_lock);

        if let Err(e) = manager.remove(guild_id).await {
            ctx.send_message(Message::Error(e.to_string())).await?;
            return Ok(());
        }

        ctx.send_message(Message::Other("**Bye bye senpai** 👉👈 🥹".to_string()))
            .await?;
    } else {
        ctx.send_message(Message::Error("Bot is not in a voice channel".to_string()))
            .await?;
        return Ok(());
    }

    Ok(())
}
