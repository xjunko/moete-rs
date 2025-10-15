// =======================================================
use poise::serenity_prelude as serenity;
use std::{env, sync::Arc};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

// =======================================================
mod commands;
mod events;
mod routines;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    match dotenv::dotenv() {
        Ok(env) => env,
        Err(err) => panic!("Failed to load enviroment: {}", err),
    };

    logging_init().await;

    // this will be used throughout the bot
    let mut state = moete_core::State::create();
    let token = state.config.discord.token.clone();

    // what prefixes to use
    let (primary_prefix, additional_prefixes) = state.config.get_prefixes();

    // options
    let framework_options = poise::FrameworkOptions {
        event_handler: |ctx, event, _framework, data: &moete_core::State| {
            Box::pin(async move {
                if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
                    events::on_ready(ctx, data_about_bot).await?;
                    {
                        // this is slightly hacked in, but it works.
                        // starts up the background tasks
                        let ctx_arc = Arc::new(ctx.clone());
                        routines::start(ctx_arc.clone(), data.clone()).await;
                    }
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
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // this loads data instantly, no need for Arc.
                state.load(ctx).await?;

                // build sql
                if let Some(pool) = state.pool.as_ref() {
                    moete_core::build_sql(pool).await?;
                }

                // poise thingamajig
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;

                Ok(state)
            })
        })
        .build();

    // client
    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::GUILD_MEMBERS
        | serenity::GatewayIntents::GUILD_PRESENCES
        | serenity::GatewayIntents::MESSAGE_CONTENT;

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
