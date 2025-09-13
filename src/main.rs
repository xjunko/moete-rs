mod builtins;
mod commands;
mod state;

use poise::serenity_prelude as serenity;
use std::sync::Arc;

use log::info;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, state::State, Error>;

async fn on_ready(_ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), Error> {
    info!("Logged in as {}", ready.user.name);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let mut state = state::State::create();

    let token = state.config.discord.token.clone();

    // get all the required param
    let prefix = state
        .config
        .discord
        .prefix
        .get(0)
        .expect("No prefix set")
        .clone();

    // abit ugly but whatevs
    let prefixes: Vec<poise::Prefix> = state
        .config
        .discord
        .prefix
        .iter()
        .map(|s| {
            // this leaks memory, but it only does so during startup, should be fine.
            poise::Prefix::Literal(Box::leak(s.clone().into_boxed_str()))
        })
        .collect();

    let intents = serenity::GatewayIntents::all();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    match event {
                        serenity::FullEvent::Ready { data_about_bot, .. } => {
                            on_ready(ctx, data_about_bot).await
                        }
                        _ => Ok(()),
                    }
                })
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(prefix.into()),
                additional_prefixes: prefixes,
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600), // ought to be enough
                ))),
                ..Default::default()
            },
            commands: {
                let mut cmds = Vec::new();
                cmds.extend(commands::utility::commands());
                cmds
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, _framework| {
            Box::pin(async move {
                state.load(ctx);
                Ok(state)
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await?;
    Ok(())
}
