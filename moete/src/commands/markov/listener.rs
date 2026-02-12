use moete_core::MoeteError;

use super::{learn, random};
use crate::serenity;

pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &moete_core::State,
) -> Result<(), MoeteError> {
    learn::on_message(ctx, message, data).await?;
    random::on_message(ctx, message, data).await?;
    Ok(())
}
