mod builtins;
mod commands;
mod core;
mod events;
mod routines;

use poise::serenity_prelude as serenity;
use std::sync::Arc;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, core::State, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let mut data = core::State::create();
    let token = data.config.discord.token.clone();
    let status = data.config.discord.status.clone();

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
                routines::start(Arc::clone(&ctx_arc), status.clone()).await;

                // this loads data instantly, no need for Arc.
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
