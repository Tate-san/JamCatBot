use std::{arch::x86_64::_SIDD_CMP_RANGES, sync::Arc};

use super::prelude::*;
use crate::music::types::TrackInfo;

#[poise::command(prefix_command, guild_only, aliases("qm", "m"), category = "Music")]
pub async fn queue_move(ctx: Context<'_>, from: usize, mut to: usize) -> Result<(), Error> {
    let call = ctx.get_bot_call().await?;
    let handler = call.lock().await;
    let queue_len = handler.queue().len();

    if from < 1 || from >= queue_len {
        return Err(BotError::Generic(format!(
            "Invalid track index, allowed range 1 - {}",
            queue_len - 1
        )));
    }

    to = to.clamp(1, queue_len - 1);

    handler.queue().modify_queue(move |queue| {
        let track = queue.remove(from).expect("Always valid");
        queue.insert(to, track);
    });

    let moved_track = handler.queue().current_queue()[to]
        .typemap()
        .read()
        .await
        .get::<TrackInfo>()
        .unwrap()
        .clone();

    ctx.send_message(Message::Success(format!(
        "Track *{}* has been successfully moved\n{from}. -> {to}.",
        moved_track.full_name()
    )))
    .await?;

    Ok(())
}

#[poise::command(prefix_command, guild_only, aliases("qr", "r"), category = "Music")]
pub async fn remove(ctx: Context<'_>, index: usize, range: Option<usize>) -> Result<(), Error> {
    let call = ctx.get_bot_call().await?;
    let handler = call.lock().await;
    let queue_len = handler.queue().len();

    if index < 1 || index >= queue_len {
        return Err(BotError::Generic(format!(
            "Invalid track index, allowed range 1 - {}",
            queue_len - 1
        )));
    }

    let index_track = handler.queue().current_queue()[index]
        .typemap()
        .read()
        .await
        .get::<TrackInfo>()
        .unwrap()
        .clone();

    let _range = range.clone().unwrap_or(1);

    if _range < 1 {
        return Err(BotError::Generic(
            "Range has to be equal or greater than 1".to_string(),
        ));
    }

    let to = index + _range;

    handler.queue().modify_queue(move |queue| {
        for _ in 0.._range {
            queue.remove(index);
        }
    });

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
