use rand::Rng;

use super::prelude::*;
use crate::api;

#[poise::command(prefix_command, slash_command, category = "NSFW")]
pub async fn naughty(
    ctx: Context<'_>,
    #[description = "Tags to search for"]
    #[rest]
    tags: Option<String>,
) -> Result<(), Error> {
    let mut redgifs = api::RedgifsApi::new()?;
    redgifs.login_temporary().await?;

    let random_page: u32 = rand::thread_rng().gen_range(1..100);

    let res = redgifs
        .search(
            random_page,
            10,
            api::redgifs::SearchOrder::Best,
            tags.clone().unwrap_or_default(),
        )
        .await;

    match res {
        Ok(list) => {
            let gif_count = list.gifs.len();
            let random_gif: usize = rand::thread_rng().gen_range(0..gif_count);

            let link = list.gifs[random_gif].urls.hd.clone().unwrap_or_default();

            ctx.say(link).await?;

            Ok(())
        }
        Err(error) => {
            tracing::error!("Tags: {:?}", tags);
            Err(error.into())
        }
    }
}

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("random", "creator"),
    subcommand_required,
    category = "NSFW"
)]
pub async fn coomer(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

#[poise::command(prefix_command, slash_command, category = "NSFW")]
pub async fn creator(
    ctx: Context<'_>,
    #[description = "Creator name"] name: String,
) -> Result<(), Error> {
    let api = api::CoomerApi::new()?;
    let creators = api.creators_cached().await?;
    let creator = match api.find_creator_by_name(&name, &creators).await {
        Some(c) => c,
        None => return Err(anyhow::anyhow!("{name} has not been found").into()),
    };

    let posts = api.creator_posts(&creator).await?;

    let mut files = vec![];

    for post in posts {
        if let Some(file) = post.file {
            files.push(file.clone());
        }

        for attachment in post.attachments {
            files.push(attachment.clone());
        }
    }

    if files.is_empty() {
        return Err(anyhow::anyhow!("{name} ({}) has no content", &creator.service).into());
    }

    let files_length = files.len();
    let random_file = rand::thread_rng().gen_range(0..files_length);

    let file = &files[random_file];

    let file_url = api.get_file_url(&file.path).await?;

    ctx.send(
        poise::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .url(&file_url)
                .image(&file_url)
                .title("Content")
                .author(
                    serenity::CreateEmbedAuthor::new(&creator.name)
                        .url(api.get_creator_url(&creator))
                        .icon_url(api.get_creator_icon_url(&creator)),
                )
                .field("Service", &creator.service, false)
                .field("Popularity", format!("{}", creator.favorited), false)
                .color(serenity::Colour::MEIBE_PINK),
        ),
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, category = "NSFW")]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    tracing::info!("Fetching creators");
    let reply = ctx.say("Searching for goodies").await?;
    let api = api::CoomerApi::new()?;
    let creators = api.creators_cached().await?;
    let creators_len = creators.len();

    loop {
        let random_creator = rand::thread_rng().gen_range(0..creators_len);

        let creator = creators[random_creator].clone();

        tracing::info!("Searching for content of {}", &creator.name);
        reply
            .edit(
                ctx,
                poise::CreateReply::default()
                    .content(format!("Browsing thru {}'s naughties", &creator.name)),
            )
            .await?;

        let posts = api.creator_posts(&creator).await?;

        let mut files = vec![];

        for post in posts {
            if let Some(file) = post.file {
                files.push(file.clone());
            }

            for attachment in post.attachments {
                files.push(attachment.clone());
            }
        }

        if files.is_empty() {
            tracing::warn!("{} has no content, trying another", &creator.name);
            reply
                .edit(
                    ctx,
                    poise::CreateReply::default()
                        .content(format!("{} has no goodies, trying another", &creator.name)),
                )
                .await?;
            continue;
        }

        let files_length = files.len();
        let random_file = rand::thread_rng().gen_range(0..files_length);

        let file = &files[random_file];
        tracing::info!("Getting file URL");

        reply
            .edit(
                ctx,
                poise::CreateReply::default().content("Found sauce, downloading"),
            )
            .await?;
        let file_url = api.get_file_url(&file.path).await?;

        tracing::info!("Serving sauce {}", file_url);

        reply
            .edit(
                ctx,
                poise::CreateReply::default()
                    .embed(
                        serenity::CreateEmbed::new()
                            .url(&file_url)
                            .image(&file_url)
                            .title("Content")
                            .author(
                                serenity::CreateEmbedAuthor::new(&creator.name)
                                    .url(api.get_creator_url(&creator))
                                    .icon_url(api.get_creator_icon_url(&creator)),
                            )
                            .field("Service", &creator.service, false)
                            .field("Popularity", format!("{}", creator.favorited), false)
                            .color(serenity::Colour::MEIBE_PINK),
                    )
                    .content(""),
            )
            .await?;

        return Ok(());
    }
}

#[poise::command(prefix_command, slash_command, category = "NSFW")]
pub async fn hentai(ctx: Context<'_>) -> Result<(), Error> {
    let api = api::WaifuApi::new()?;
    let image = api.search(true, None).await?;

    ctx.say(image.url).await?;

    Ok(())
}
