use std::collections::HashMap;

use once_cell::sync::Lazy;
use serenity::all::{CreateWebhook, WebhookType};
use tokio::sync::Mutex;

use crate::serenity;

// cache
// as much as i like to avoid global state, this is the easiest way to cache webhooks
static CACHE: Lazy<Mutex<HashMap<serenity::ChannelId, serenity::Webhook>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub async fn get_or_create_webhook(
    ctx: &serenity::Context,
    channel_id: serenity::ChannelId,
) -> Option<serenity::Webhook> {
    // check cache first
    let mut cache = CACHE.lock().await;
    if let Some(cached) = cache.get(&channel_id) {
        return Some(cached.clone());
    }

    let webhook = match channel_id.webhooks(&ctx.http).await {
        Ok(hooks) => {
            // Look for an existing webhook named "Moete" and of type Incoming
            if let Some(existing) = hooks.into_iter().find(|hook| {
                hook.kind == WebhookType::Incoming
                    && hook.name.as_deref() == Some("Moete")
                    && hook.user.as_ref().unwrap().id == ctx.cache.current_user().id
            }) {
                // FIXME: do we really need to do this?
                let webhook_with_token = ctx.http.get_webhook(existing.id).await.ok()?;
                Some(webhook_with_token)
            } else {
                // no matching webhook found, try to create one
                channel_id
                    .create_webhook(
                        &ctx.http,
                        CreateWebhook::new("Moete").audit_log_reason(
                            "Moete requires an incoming webhook named 'Moete' to send messages with custom emotes.",
                        ),
                    )
                    .await
                    .ok()
            }
        },
        Err(_) => {
            // couldn’t list webhooks (no permission) -> try creating one
            channel_id
                .create_webhook(
                    &ctx.http,
                    CreateWebhook::new("Moete").audit_log_reason(
                        "Moete requires an incoming webhook named 'Moete' to send messages with custom emotes.",
                    ),
                )
                .await
                .ok()
        },
    };

    // cache
    if let Some(ref hook) = webhook {
        cache.insert(channel_id, hook.clone());
    }

    webhook
}
