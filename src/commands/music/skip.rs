use super::prelude::*;

#[poise::command(prefix_command, guild_only, aliases("s"), category = "Music")]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap().clone();
    let guild_id = guild.id.clone();

    let manager = &ctx.data().songbird;
    let has_handler = manager.get(guild_id.clone()).is_some();

    if has_handler {
        let user_voice_state = if let Some(voice_state) = guild.voice_states.get(&ctx.author().id) {
            voice_state
        } else {
            ctx.send_message(
                Message::Error("You have to be in a voice channel".to_string()).into(),
            )
            .await?;
            return Ok(());
        };

        let call = manager.get(guild_id).unwrap();

        let call_lock = call.lock().await;

        let user_channel = user_voice_state.channel_id.unwrap();
        let bot_channel = call_lock.current_channel().unwrap();

        if user_channel.get() != bot_channel.0.get() {
            ctx.send_message(
                Message::Error("You are not in the same voice channel as bot".to_string()).into(),
            )
            .await?;
            return Ok(());
        }

        if let Err(error) = call_lock.queue().skip() {
            let message = format!("Unable to skip the track: {error}");

            tracing::error!(message);
            ctx.send_message(Message::Error(message).into()).await?;
        }
    } else {
        ctx.send_message(Message::Error("Bot is not in a voice channel".to_string()).into())
            .await?;
        return Ok(());
    }

    Ok(())
}
