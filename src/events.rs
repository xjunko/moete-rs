use tracing::info;

use crate::Error;
use crate::commands;
use crate::core;
use crate::serenity;

pub async fn on_ready(_ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), Error> {
    info!("Logged in as {}", ready.user.name);

    Ok(())
}

pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &core::State,
) -> Result<(), Error> {
    commands::emote::listener::on_message(ctx, message, data).await?;
    commands::markov::listener::on_message(ctx, message, data).await?;
    Ok(())
}
