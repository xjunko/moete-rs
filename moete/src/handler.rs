use std::sync::Arc;

use poise::FrameworkContext;
use serenity::all::prelude::Context;
use serenity::client::FullEvent;

use crate::{
    events,
    routines,
};

pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, moete_core::State, moete_core::MoeteError>,
    data: &moete_core::State,
) -> Result<(), moete_core::MoeteError> {
    match event {
        FullEvent::Ready { data_about_bot, .. } => {
            events::on_ready(ctx, data_about_bot).await?;
            {
                // this is slightly hacked in, but it works.
                // starts up the background tasks
                let ctx_arc = Arc::new(ctx.clone());
                routines::start(ctx_arc.clone(), data.clone()).await;
            }
        },
        FullEvent::Message { new_message } => {
            events::on_message(ctx, new_message, data).await?;
        },
        _ => {},
    }

    Ok(())
}

pub async fn on_error(
    error: poise::FrameworkError<'_, moete_core::State, moete_core::MoeteError>,
) {
    match error {
        poise::FrameworkError::UnknownCommand { .. } => { /* noop */ },

        poise::FrameworkError::ArgumentParse { ctx, input, error, .. } => {
            let usage = match &ctx.command().help_text {
                Some(help_text) => &**help_text,
                None => "Please check the help menu for usage information",
            };

            let response = if let Some(input) = input {
                format!(
                    "**Cannot parse `{}` as argument: {}**\n{}",
                    input, error, usage
                )
            } else {
                format!("**{}**\n{}", error, usage)
            };

            if response.contains("Too many arguments were passed")
                || response.contains("Too few arguments were passed")
            {
                let command_name = {
                    if let Some(parent) = ctx.parent_commands().last() {
                        format!("{} {}", parent.name, ctx.command().name)
                    } else {
                        ctx.command().name.to_string()
                    }
                };

                // custom handler for this scenario
                let _ = moete_discord::help::help(
                    ctx,
                    Some(&command_name),
                    moete_discord::help::HelpConfiguration::default(),
                )
                .await;
            } else {
                let _ = ctx.say(response).await;
            }
        },

        _ => {
            let _ = poise::builtins::on_error(error).await;
        },
    }
}
