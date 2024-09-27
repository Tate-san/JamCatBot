use super::prelude::*;

#[poise::command(prefix_command, guild_only, aliases("cq"), category = "Music")]
pub async fn clear_queue(ctx: Context<'_>) -> Result<(), Error> {
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

        if call_lock.queue().len() <= 1 {
            ctx.send_message(Message::Error("There's nothing to clear".to_string()))
                .await?;
            return Ok(());
        }

        call_lock.queue().modify_queue(|q| drop(q.drain(1..)));

        ctx.send_message(Message::Success(
            "Queue has been successfully cleared".to_string(),
        ))
        .await?;
    } else {
        ctx.send_message(Message::Error("Bot is not in a voice channel".to_string()))
            .await?;
        return Ok(());
    }

    Ok(())
}
