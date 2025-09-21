use super::ALLOWED;
use crate::core;
use crate::{ConnectionPool, Error, serenity};

const COMMON_BOT_PREFIXES: &[&str] = &[
    ";", "t!", "pls ", "please ", "p ", "->", "!", "`", "``", ";;", "~>", ">", "<", "$", "k!",
    ".calc", ".ss", ".google", ".",
];

pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &core::State,
) -> Result<(), Error> {
    if !ALLOWED.contains(&message.author.id.into()) {
        return Ok(());
    }

    if let Some(pool) = {
        let data_read = ctx.data.read().await;
        data_read.get::<ConnectionPool>().unwrap().clone()
    } {
        let (main_prefix, _) = data.config.get_prefixes();
        // Checks to make sure the content of the message met certain criteria

        // If starts with prefixes, ignore.
        if message.content.starts_with(&main_prefix) {
            return Ok(());
        }

        // Empty message, ignore.
        if message.content.trim().is_empty() {
            return Ok(());
        }

        // More checks
        if COMMON_BOT_PREFIXES
            .iter()
            .any(|prefix| message.content.starts_with(prefix))
        {
            return Ok(());
        }

        core::markov::add_message(&pool, message.author.id.into(), &message.content).await?;
    }

    Ok(())
}
