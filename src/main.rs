// =======================================================
use poise::serenity_prelude as serenity;
use std::{env, sync::Arc};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// =======================================================
mod builtins;
mod commands;
mod core;
mod events;
mod routines;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, core::State, Error>;

#[tokio::main]
async fn main() {
    match dotenv::dotenv() {
        Ok(env) => env,
        Err(err) => panic!("Failed to load enviroment: {}", err),
    };

    logging_init().await;

    // this will be used throughout the bot
    let mut state = core::State::create();
    let token = state.config.discord.token.clone();

    // what prefixes to use
    let (primary_prefix, additional_prefixes) = state.config.get_prefixes();

    // options
    let framework_options = poise::FrameworkOptions {
        event_handler: |ctx, event, _framework, data| {
            Box::pin(async move {
                if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
                    events::on_ready(ctx, data_about_bot).await?;
                }

                if let serenity::FullEvent::Message { new_message } = event {
                    events::on_message(ctx, new_message, data).await?;
                }

                Ok(())
            })
        },
        on_error: |err| {
            Box::pin(async move {
                let _ = poise::builtins::on_error(err).await;
            })
        },
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some(primary_prefix),
            additional_prefixes,
            edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                std::time::Duration::from_secs(3600),
            ))),
            ..Default::default()
        },
        commands: {
            let mut cmds = Vec::new();
            cmds.extend(commands::commands());
            cmds
        },
        initialize_owners: true,
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(framework_options)
        .setup(|ctx, _ready, _framework| {
            Box::pin(async move {
                // background tasks
                let ctx_arc = Arc::new(ctx.clone());
                let config_clone = Arc::clone(&state.config);
                routines::start(Arc::clone(&ctx_arc), config_clone).await;

                // this loads data instantly, no need for Arc.
                state.load(ctx).await?;
                Ok(state)
            })
        })
        .build();

    // client
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    info!("Starting Moete..");

    client.unwrap().start().await.unwrap();
}

async fn logging_init() {
    let filter = EnvFilter::from_env("MOETE_FILTER");
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    info!(
        "Logger ready - Filter {:?}",
        env::var("MOETE_FILTER").unwrap()
    );
}
