use jamcat_error::MusicError;

use super::prelude::*;

#[poise::command(prefix_command, guild_only, aliases("qm", "m"), category = "Music")]
pub async fn queue_move(ctx: Context<'_>, from: usize, mut to: usize) -> Result<(), Error> {
    let queue = ctx.get_queue().await?;
    let mut queue_handle = queue.lock().await;

    let queue_len = queue_handle.queue().len();

    if queue_len <= 0 {
        return Err(MusicError::QueueEmpty.into());
    }

    if from < 1 || from > queue_len {
        return Err(BotError::Generic(format!(
            "Invalid track index, allowed range 1 - {}",
            queue_len
        )));
    }

    to = to.clamp(1, queue_len);

    let moved_track = queue_handle.queue()[from - 1].clone();
    queue_handle.move_track(from - 1, to - 1)?;

    ctx.send_message(Message::Success(format!(
        "Track *{}* has been successfully moved\n{from}. -> {to}.",
        moved_track.full_name()
    )))
    .await?;

    Ok(())
}

#[poise::command(prefix_command, guild_only, aliases("qr", "r"), category = "Music")]
pub async fn remove(ctx: Context<'_>, index: usize, range: Option<usize>) -> Result<(), Error> {
    let queue = ctx.get_queue().await?;
    let mut queue_handle = queue.lock().await;

    let queue_len = queue_handle.queue().len();

    if queue_len <= 0 {
        return Err(MusicError::QueueEmpty.into());
    }

    if index < 1 || index > queue_len {
        return Err(BotError::Generic(format!(
            "Invalid track index, allowed range 1 - {}",
            queue_len
        )));
    }
    
    let index_track = queue_handle.queue()[index - 1].clone();

    let _range = range.unwrap_or(1);

    if _range < 1 {
        return Err(BotError::Generic(
            "Range has to be equal or greater than 1".to_string(),
        ));
    }

    let to = (index - 1) + _range;

    for _ in 0.._range {
        queue_handle.remove(index - 1);
    }

    if range.is_some() {
        ctx.send_message(Message::Success(format!(
            "Removed multiple tracks {index} - {} from queue",
            to - 1
        )))
        .await?;
    } else {
        ctx.send_message(Message::Success(format!(
            "Removed track *{}* on index {index} from queue",
            index_track.full_name()
        )))
        .await?;
    }

    Ok(())
}
