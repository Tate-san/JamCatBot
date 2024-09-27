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
        }
        Err(error) => {
            tracing::error!("Invalid tags: {:?}", tags);
            return Err(BotError::Generic(error.to_string()));

            //ctx.send_message(Message::Error(error.to_string())).await?;
        }
    }

    Ok(())
}

async fn coomer_creator_random_image(
    api: &api::CoomerApi,
    creator: api::coomer::types::CreatorInfo,
) -> anyhow::Result<String> {
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

    // Yeet out all videos
    let files = files
        .iter()
        .filter_map(|item| {
            if item.path.ends_with("png")
                || item.path.ends_with("jpg")
                || item.path.ends_with("jpeg")
            {
                Some(item.clone())
            } else {
                None
            }
        })
        .collect::<Vec<api::coomer::types::FileInfo>>();

    if files.is_empty() {
        return Err(anyhow::anyhow!(
            "{} ({}) has no content",
            &creator.name,
            &creator.service
        ));
    }

    let files_length = files.len();
    let random_file = rand::thread_rng().gen_range(0..files_length);

    let file = &files[random_file];

    Ok(api.get_file_url(&file.path).await?)
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
        None => {
            ctx.send_message(Message::Error(format!("{name} has not been found")))
                .await?;

            return Ok(());
        }
    };

    let image_url = match coomer_creator_random_image(&api, creator.clone()).await {
        Ok(url) => url,
        Err(error) => {
            ctx.send_message(Message::Error(error.to_string())).await?;
            return Ok(());
        }
    };

    let creator_url = api.get_creator_url(&creator);
    let creator_icon_url = api.get_creator_icon_url(&creator);

    let embed = messages::factory::create_coomer_image_embed(
        &creator,
        image_url,
        creator_url,
        creator_icon_url,
    );

    ctx.send_embed(embed).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, category = "NSFW")]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    let api = api::CoomerApi::new()?;
    let creators = api.creators_cached().await?;
    let creators_len = creators.len();

    loop {
        let random_creator = rand::thread_rng().gen_range(0..creators_len);

        let creator = creators[random_creator].clone();

        let image_url = match coomer_creator_random_image(&api, creator.clone()).await {
            Ok(url) => url,
            Err(_) => {
                continue;
            }
        };

        let creator_url = api.get_creator_url(&creator);
        let creator_icon_url = api.get_creator_icon_url(&creator);

        let embed = messages::factory::create_coomer_image_embed(
            &creator,
            image_url,
            creator_url,
            creator_icon_url,
        );

        ctx.send_embed(embed).await?;

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
