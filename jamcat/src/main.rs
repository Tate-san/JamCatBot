use std::{env, sync::Arc};

use dotenv;
use jamcat_common::types::guild::GuildCacheMap;
use tokio::sync::Mutex;
use jamcat_core::prelude::*;
use jamcat_common::types::{Data, Error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenv::dotenv();
    tracing_subscriber::fmt::init();

    let discord_token = env::var("DISCORD_TOKEN").expect("Expected discord token in the environment");
    let intents = serenity::GatewayIntents::all();
    let voice_manager = songbird::Songbird::serenity();
    let voice_manager_clone = voice_manager.clone();
    if let Err(error) = jamcat_music::providers::spotify::SPOTIFY.lock().await.auth().await {
        tracing::error!("Unable to auth spotify: {error}")
    }

    let commands = vec![
        // General
        jamcat_commands::general::help(),
        jamcat_commands::general::test(),

        // Animals
        jamcat_commands::animals::cat(),
        jamcat_commands::animals::dog(),

        // Media
        jamcat_commands::media::video(),

        // Fun
        jamcat_commands::fun::yapper(),
        jamcat_commands::fun::flip(),

        // NSFW
        jamcat_commands::nsfw::coomer::coomer(),
        jamcat_commands::nsfw::coomer::creator(),
        jamcat_commands::nsfw::coomer::random(),
        jamcat_commands::nsfw::hentai::hentai(),
        jamcat_commands::nsfw::redgifs::naughty(),

        // Music
        jamcat_commands::music::join(),
        jamcat_commands::music::leave(),

        jamcat_commands::music::play(),
        jamcat_commands::music::stop(),
        jamcat_commands::music::loops(),
        jamcat_commands::music::now_playing(),
        jamcat_commands::music::pause(),
        jamcat_commands::music::queue(),
        jamcat_commands::music::queue_move(),
        jamcat_commands::music::clear_queue(),
        jamcat_commands::music::remove(),
        jamcat_commands::music::resume(),
        //jamcat_commands::music::seek(),
        jamcat_commands::music::skip(),
        jamcat_commands::music::volume(),

    ];

    // Framework setup
    let framework: poise::Framework<Data, Error> = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(".".into()),
                ..Default::default()
            },
            event_handler: |ctx, event,framework, data| {
                Box::pin(jamcat_core::handlers::serenity::event_handler(ctx, event, framework, data))
            },
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = jamcat_core::handlers::serenity::on_error(error).await {
                        tracing::error!("Error while handling error: {}", e);
                    }
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(jamcat_common::types::Data {
                    http: reqwest::ClientBuilder::new()
                        .use_rustls_tls()
                        .cookie_store(true)
                        .build()
                        .expect("Failed to build reqwest client"),
                    songbird: voice_manager_clone,
                    guild_cache: Arc::new(Mutex::new(GuildCacheMap::default())),
                })
            })
        })
        .build();

    // Client go brrrrr
    let mut client = serenity::client::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .voice_manager_arc(voice_manager)
        .await?;

    client.start().await?;

    anyhow::Ok(())
}
