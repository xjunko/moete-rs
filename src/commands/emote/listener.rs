use log::error;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::Error;
use crate::core;
use crate::serenity;

static RE_EMOTE_TYPED: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(\B(:|;|\.)[a-zA-Z0-9_-]+(:|;|\.)\B)").unwrap());
static RE_EMOTE_CLEAN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(:|;|\.)").unwrap());

pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &core::State,
) -> Result<(), Error> {
    let mut found_emote = false;
    let mut words: Vec<String> = message
        .content
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    for word in &mut words {
        if RE_EMOTE_TYPED.is_match(word) {
            let clean = RE_EMOTE_CLEAN.replace_all(word, "").into_owned();
            if let Some(emote) = data.emotes.get(&clean) {
                *word = emote.to_string();
                found_emote = true;
            }
        }
    }

    if found_emote {
        let converted: String = words.join(" ");

        if let Err(err) = message.delete(&ctx.http).await {
            error!("Failed to delete message: {err:?}");
        }

        if let Err(err) = message.channel_id.say(&ctx.http, converted).await {
            error!("Failed to send message: {err:?}");
        }
    }

    Ok(())
}
