use serenity::all::CreateWebhook;
use serenity::all::WebhookType;

use crate::serenity;

pub async fn get_or_create_webhook(
    ctx: &serenity::Context,
    channel_id: serenity::ChannelId,
) -> Option<serenity::Webhook> {
    match channel_id.webhooks(&ctx.http).await {
        Ok(hooks) => {
            // Look for an existing webhook named "Moete" and of type Incoming
            if let Some(existing) = hooks.into_iter().find(|hook| {
                hook.kind == WebhookType::Incoming && hook.name.as_deref() == Some("Moete")
            }) {
                Some(existing)
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
        }
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
        }
    }
}
