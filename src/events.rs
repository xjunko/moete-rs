use super::Error;
use super::serenity;
use crate::core;
use tracing::info;

pub async fn on_ready(_ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), Error> {
    info!("Logged in as {}", ready.user.name);
    Ok(())
}

pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &core::State,
) -> Result<(), Error> {
    crate::commands::emote::listener::on_message(ctx, message, data).await?;
    Ok(())
}
