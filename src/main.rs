mod config;
mod state;
pub use state::State;

use poise::serenity_prelude as serenity;
use std::sync::Arc;

use log::info;

mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

async fn on_ready(_ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), Error> {
    info!("Logged in as {}", ready.user.name);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
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
                prefix: Some(";".into()),
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
        .setup(|ctx, _ready, _framework| Box::pin(async move { Ok(State::load(ctx)) }))
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await?;
    Ok(())
}
