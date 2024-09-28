mod api;
mod commands;
pub mod error;
mod handlers;
pub mod messages;
mod music;
mod poise_extension;
mod prelude;
mod sources;
mod types;
mod utils;
mod voice;

use prelude::*;
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use types::guild::GuildCacheMap;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::all();
    let manager = songbird::Songbird::serenity();
    let manager_clone = manager.clone();
    if let Err(error) = sources::spotify::SPOTIFY.lock().await.auth().await {
        tracing::error!("Unable to auth spotify: {error}")
    }

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            on_error: |error| {
                Box::pin(async move {
                    if let Err(e) = handlers::serenity::on_error(error).await {
                        tracing::error!("Error while handling error: {}", e);
                    }
                })
            },
            commands: vec![
                commands::general::help(),
                commands::general::test(),
                commands::animals::cat(),
                commands::animals::dog(),
                commands::naughty::naughty(),
                commands::naughty::coomer(),
                commands::naughty::hentai(),
                commands::fun::yapper(),
                commands::music::play(),
                commands::music::stop(),
                commands::music::skip(),
                commands::music::clear_queue(),
                commands::music::volume(),
                commands::music::queue(),
                commands::music::queue_move(),
                commands::music::remove(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(".".into()),
                ..Default::default()
            },
            event_handler: |ctx, event, framework, data| {
                Box::pin(handlers::serenity::event_handler(
                    ctx, event, framework, data,
                ))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(types::Data {
                    http: reqwest_old::ClientBuilder::new()
                        .use_rustls_tls()
                        .cookie_store(true)
                        .build()
                        .expect("Failed to build reqwest client"),
                    songbird: manager_clone,
                    guild_cache: Arc::new(Mutex::new(GuildCacheMap::default())),
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .voice_manager_arc(manager)
        .await
        .unwrap();

    client.start().await.unwrap();
}
