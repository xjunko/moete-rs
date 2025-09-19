use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::ExecuteWebhook;
use tracing::error;

use crate::Error;
use crate::builtins;
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
        let user = message.author.clone();
        let converted: String = words.join(" ");

        // Failing to delete message is not a big deal
        if let Err(err) = message.delete(&ctx.http).await {
            error!("Failed to delete message: {err:?}");
        }

        // Try to send thru webhook, if failed, send thru text
        if let Some(webhook) =
            builtins::discord::webhook::get_or_create_webhook(ctx, message.channel_id).await
        {
            if let Err(err) = webhook
                .execute(
                    &ctx.http,
                    true,
                    ExecuteWebhook::new()
                        .content(converted.clone())
                        .username(user.display_name())
                        .avatar_url(user.avatar_url().unwrap_or(user.default_avatar_url())),
                )
                .await
            {
                error!("Failed to execute webhook: {err:?}");
            } else {
                return Ok(());
            }
        }

        // Webhook failed, fallback to normal message
        if let Err(err) = message
            .channel_id
            .say(
                &ctx.http,
                format!("{} - {}", converted.clone(), user.display_name()),
            )
            .await
        {
            error!("Failed to send message: {err:?}");
        }
    }

    Ok(())
}
