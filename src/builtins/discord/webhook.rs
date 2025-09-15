use serenity::all::CreateWebhook;

use crate::serenity;

pub async fn get_or_create_webhook(
    ctx: &serenity::Context,
    channel_id: serenity::ChannelId,
) -> Option<serenity::Webhook> {
    match channel_id.webhooks(&ctx.http).await {
        Ok(mut hooks) => {
            if let Some(existing) = hooks.pop() {
                Some(existing)
            } else {
                // no webhook found, try to create one
                channel_id
                    .create_webhook(
                        &ctx.http,
                        CreateWebhook::new("Moete").audit_log_reason(
                            "Moete requires a webhook to send a message with custom emotes.",
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
                        "Moete requires a webhook to send a message with custom emotes.",
                    ),
                )
                .await
                .ok()
        }
    }
}
