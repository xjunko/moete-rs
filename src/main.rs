mod config;
mod state;
pub use state::State;

use poise::serenity_prelude as serenity;
use std::sync::Arc;

use log::info;

mod commands;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

async fn on_ready(
    _ctx: &serenity::prelude::Context,
    _ready: &serenity::Ready,
    _framework: &poise::Framework<State, Error>,
) -> Result<(), Error> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set");
    let intents = serenity::GatewayIntents::all();
    let framework = poise::Framework::builder()
        .setup(|ctx, ready, framework| {
            Box::pin(async move {
                info!("Account ID:: {}", ready.user.id);
                info!("Name: {}", ready.user.name);
                on_ready(ctx, ready, framework).await?;
                info!("Should be ready now!");
                Ok(State::load(ctx))
            })
        })
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(";".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600), // ought to be enough
                ))),
                ..Default::default()
            },
            commands: vec![commands::ping(), commands::urban()],
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await?;
    Ok(())
}
