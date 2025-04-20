use jamcat_common::types::music::TrackInfo;

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
    // Substracted by 1 to ignore the current playing song
    let total_pages = jamcat_utils::get_total_pages(current_queue.len() - 1, PAGE_ITEMS_COUNT);

    let reply = poise::CreateReply::default().embed(message::create_queue_list_embed(
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
                options: create_menu_options(total_pages),
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
        // Substracted by 1 to ignore the current playing song
        let total_pages = jamcat_utils::get_total_pages(current_queue.len() - 1, PAGE_ITEMS_COUNT);

        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(message::create_queue_list_embed(
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
    let queue = ctx.get_queue().await?;
    let mut queue_handle = queue.lock().await;
    let mut current_queue: Vec<TrackInfo> = queue_handle.queue().iter().cloned().collect();

    if let Some(current) = queue_handle.current_track_info() {
        current_queue.insert(0, current);
    }

    Ok(current_queue)
}
