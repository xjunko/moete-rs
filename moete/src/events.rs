use moete_core::MoeteError;
use tracing::info;

use crate::{commands, serenity};

pub async fn on_ready(_ctx: &serenity::Context, ready: &serenity::Ready) -> Result<(), MoeteError> {
    info!("Logged in as {}", ready.user.name);

    Ok(())
}

pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &moete_core::State,
) -> Result<(), MoeteError> {
    commands::emote::listener::on_message(ctx, message, data).await?;
    commands::markov::listener::on_message(ctx, message, data).await?;
    commands::counter::listener::on_message(ctx, message, data).await?;
    commands::role::housekeeping::on_message(ctx, message, data).await?;
    commands::user_macros::listener::on_message(ctx, message, data).await?;
    Ok(())
}
