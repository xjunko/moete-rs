use super::ALLOWED;
use crate::core;
use crate::{Error, serenity};

const COMMON_BOT_PREFIXES: &[&str] = &[
    ";", "t!", "pls ", "please ", "p ", "->", "!", "`", "``", ";;", "~>", ">", "<", "$", "k!",
    ".calc", ".ss", ".google", ".",
];

pub async fn on_message(
    _ctx: &serenity::Context,
    message: &serenity::Message,
    data: &core::State,
) -> Result<(), Error> {
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

        core::markov::add_message(pool, message.author.id.into(), &message.content).await?;
    }

    Ok(())
}
