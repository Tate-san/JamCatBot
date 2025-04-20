use jamcat_utils::api::coomer::*;
use crate::prelude::*;

async fn coomer_creator_random_image(
    api: &CoomerApi,
    creator: model::CreatorInfo,
) -> Result<String, Error> {
    let posts = match api.creator_posts(&creator).await {
        Ok(posts) => posts,
        Err(e) => {
            tracing::error!("Unable to fetch creator posts: {e}");
            return Err(e.into());
        }
    };

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
        .collect::<Vec<model::FileInfo>>();

    if files.is_empty() {
        return Err(Error::Generic(format!(
            "{} ({}) has no content",
            &creator.name,
            &creator.service
        )));
    }

    let files_length = files.len();


    let random_file = rand::rng().random_range(0..files_length);
    let file = &files[random_file];

    match api.get_file_url(file.clone()).await {
        Ok(path) => Ok(path),
        Err(e) => {
            tracing::error!("Unable to get post url {e}");
            Err(e.into())
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
    let msg_handle = ctx
        .send_message(Message::Other("Getting the sauce".to_string()))
        .await?;

    let api = CoomerApi::new()?;
    let creators = api.creators_cached().await?;
    let creator = match api.find_creator_by_name(&name, &creators).await {
        Some(c) => c,
        None => {
            msg_handle.delete(ctx).await?;
            ctx.send_message(Message::Error(format!("{name} has not been found")))
                .await?;

            return Ok(());
        }
    };

    let image_url = match coomer_creator_random_image(&api, creator.clone()).await {
        Ok(url) => url,
        Err(error) => {
            msg_handle.delete(ctx).await?;
            ctx.send_message(Message::Error(error.to_string())).await?;
            return Ok(());
        }
    };

    let creator_url = api.get_creator_url(&creator);
    let creator_icon_url = api.get_creator_icon_url(&creator);

    let embed = message::create_coomer_image_embed(
        &creator,
        image_url,
        creator_url,
        creator_icon_url,
    );

    msg_handle
        .edit(ctx, poise::CreateReply::default().embed(embed))
        .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command, category = "NSFW")]
pub async fn random(ctx: Context<'_>) -> Result<(), Error> {
    let msg_handle = ctx
        .send_message(Message::Other("Getting the sauce".to_string()))
        .await?;

    let api = CoomerApi::new()?;
    let creators = api.creators_cached().await?;
    let creators_len = creators.len();

    loop {
        let random_creator = rand::rng().random_range(0..creators_len);

        let creator = creators[random_creator].clone();

        let image_url = match coomer_creator_random_image(&api, creator.clone()).await {
            Ok(url) => url,
            Err(_) => {
                continue;
            }
        };

        let creator_url = api.get_creator_url(&creator);
        let creator_icon_url = api.get_creator_icon_url(&creator);

        let embed = message::create_coomer_image_embed(
            &creator,
            image_url,
            creator_url,
            creator_icon_url,
        );

        //ctx.send_embed(embed).await?;
        msg_handle
            .edit(ctx, poise::CreateReply::default().embed(embed))
            .await?;

        return Ok(());
    }
}