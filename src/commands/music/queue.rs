use crate::music::types::TrackInfo;

use super::prelude::*;

const PAGE_ITEMS_COUNT: usize = 10;

#[poise::command(prefix_command, guild_only, aliases("q"), category = "Music")]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let current_queue = current_queue_tracks(&ctx).await?;

    if current_queue.is_empty() {
        ctx.send_message(Message::Other("Queue is empty".to_string()))
            .await?;

        return Ok(());
    }

    let ctx_id = ctx.id();
    let select_menu_id = format!("{ctx_id}select_menu");

    let mut current_page: usize = 0;
    let total_pages = (current_queue.len() / PAGE_ITEMS_COUNT) + 1;

    let reply = poise::CreateReply::default().embed(messages::factory::create_queue_list_embed(
        current_queue,
        0,
        PAGE_ITEMS_COUNT,
    ));

    if total_pages <= 1 {
        ctx.send(reply).await?;
        return Ok(());
    }

    ctx.send(reply.components(vec![serenity::CreateActionRow::SelectMenu(
        serenity::CreateSelectMenu::new(
            select_menu_id.clone(),
            serenity::CreateSelectMenuKind::String {
                options: create_menu_options(total_pages.clone()),
            },
        ),
    )]))
    .await?;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(60 * 10))
        .await
    {
        if press.data.custom_id == select_menu_id {
            if let serenity::ComponentInteractionDataKind::StringSelect { values } =
                &press.data.kind
            {
                current_page = values[0].parse().expect("Always gonna be a number");
            }
        }

        let current_queue = current_queue_tracks(&ctx).await?;
        let total_pages = (current_queue.len() / PAGE_ITEMS_COUNT) + 1;

        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(messages::factory::create_queue_list_embed(
                            current_queue,
                            current_page,
                            PAGE_ITEMS_COUNT,
                        ))
                        .components(vec![serenity::CreateActionRow::SelectMenu(
                            serenity::CreateSelectMenu::new(
                                select_menu_id.clone(),
                                serenity::CreateSelectMenuKind::String {
                                    options: create_menu_options(total_pages),
                                },
                            ),
                        )]),
                ),
            )
            .await?;
    }

    Ok(())
}

fn create_menu_options(pages: usize) -> Vec<serenity::CreateSelectMenuOption> {
    let mut options = vec![];

    for i in 0..pages {
        options.push(serenity::CreateSelectMenuOption::new(
            format!("Page {}", i + 1),
            i.to_string(),
        ));
    }

    options
}

async fn current_queue_tracks(ctx: &Context<'_>) -> Result<Vec<TrackInfo>, Error> {
    let call = ctx.get_bot_call().await?;
    let handle = call.lock().await;

    let mut tracks = vec![];

    for (idx, track) in handle.queue().current_queue().iter().enumerate() {
        let typemap = track.typemap().read().await;

        let track_info = typemap
            .get::<TrackInfo>()
            .expect("Should be known at this point")
            .clone();

        tracks.push(track_info.clone());
    }

    Ok(tracks)
}
