use std::time::Duration;

use super::prelude::*;

#[poise::command(prefix_command, guild_only, category = "Music")]
pub async fn seek(ctx: Context<'_>, seconds: u32) -> Result<(), Error> {
    let call = ctx.get_bot_call().await?;
    let handle = call.lock().await;

    if let Some(handle) = handle.queue().current() {
        let info = handle
            .get_info()
            .await
            .map_err(|e| BotError::Generic(e.to_string()))?;

        let seek_to = Duration::from_secs(seconds.into());

        if info.position > seek_to {
            return Err(BotError::Generic(
                "Can't go back in time u dumb quack 🤦🏿".to_string(),
            ));
        }

        handle
            .seek_async(seek_to)
            .await
            .map_err(|e| BotError::Generic(e.to_string()))?;

        Ok(())
    } else {
        Err(BotError::Generic(
            "Can't seek the nonexisting song".to_string(),
        ))
    }
}
