use moete_core::MoeteError;

use super::{
    learn,
    random,
};
use crate::serenity;

pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &moete_core::State,
) -> Result<(), MoeteError> {
    if let Some(guild_id) = message.guild_id
        && let Some(database) = data.database.as_ref()
    {
        let configuration = moete_infra::services::configuration::get(
            database,
            guild_id.into(),
        )
        .await?;

        if configuration.server.allow_markov_learning.value {
            learn::on_message(ctx, message, data).await?;
        }

        if configuration.server.allow_markov_random.value {
            random::on_message(ctx, message, data).await?;
        }
    }

    Ok(())
}
