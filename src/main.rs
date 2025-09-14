mod builtins;
mod commands;
mod core;

use std::sync::Arc;

use log::info;
use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, core::State, Error>;

async fn on_ready(_ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), Error> {
    info!("Logged in as {}", ready.user.name);
    Ok(())
}

async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &core::State,
) -> Result<(), Error> {
    crate::commands::emote::listener::on_message(ctx, message, data).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let mut data = core::State::create();
    let token = data.config.discord.token.clone();

    // what prefixes to use
    let (primary_prefix, additional_prefixes) = {
        if data.config.discord.debug {
            builtins::prefixes::debug_prefixes()
        } else {
            builtins::prefixes::release_prefixes()
        }
    };

    // options
    let framework_options = poise::FrameworkOptions {
        event_handler: |ctx, event, _framework, data| {
            Box::pin(async move {
                if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
                    on_ready(ctx, data_about_bot).await?;
                }

                if let serenity::FullEvent::Message { new_message } = event {
                    on_message(ctx, &new_message, data).await?;
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
            prefix: Some(primary_prefix.into()),
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
                data.load(ctx).await?;
                Ok(data)
            })
        })
        .build();

    // client
    serenity::ClientBuilder::new(token, serenity::GatewayIntents::all())
        .framework(framework)
        .await?
        .start()
        .await?;

    Ok(())
}
