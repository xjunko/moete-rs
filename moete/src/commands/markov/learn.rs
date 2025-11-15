use crate::serenity;
use moete_core::MoeteError;

use super::ALLOWED;

const COMMON_BOT_PREFIXES: &[&str] = &[
    ";", "t!", "pls ", "please ", "p ", "->", "!", "`", "``", ";;", "~>", ">", "<", "$", "k!",
    ".calc", ".ss", ".google", ".",
];

/// Process a message for learning into the Markov chain.
pub async fn on_message(
    _: &serenity::Context,
    message: &serenity::Message,
    data: &moete_core::State,
) -> Result<(), MoeteError> {
    if !ALLOWED.contains(&message.author.id.into()) {
        return Ok(());
    }

    if let Some(pool) = data.pool.as_ref() {
        let (main_prefix, additional_prefixes) = data.config.get_prefixes();
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
            .chain(additional_prefixes.iter().map(|s| match s {
                poise::Prefix::Literal(s) => s,
                _ => panic!("Expecting Literal prefixes, received Regex"),
            }))
            .any(|prefix| message.content.starts_with(prefix))
        {
            return Ok(());
        }

        moete_core::markov::add_message(pool, message.author.id.into(), &message.content).await?;
    }

    Ok(())
}
