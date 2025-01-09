mod api;
mod commands;
mod constants;
mod database;
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
use sqlx::{migrate::Migrator, sqlite::SqlitePool};
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use types::guild::GuildCacheMap;

static MIGRATOR: Migrator = sqlx::migrate!();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().expect("Failed to load .env file");
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL var");
    let pool = SqlitePool::connect(&format!("sqlite://{database_url}?mode=rwc")).await?;
    MIGRATOR.run(&pool).await?;

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
                commands::music::now_playing(),
                //commands::music::seek(),
                commands::music::loops(),
                commands::music::resume(),
                commands::music::pause(),
                commands::music::join(),
                commands::music::leave(),
                commands::fun::video(),
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
                    pool: Arc::new(Mutex::new(pool)),
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .voice_manager_arc(manager)
        .await?;

    client.start().await?;

    anyhow::Ok(())
}
