use serenity::all::CreateWebhook;

use crate::serenity;

pub async fn get_or_create_webhook(
    ctx: &serenity::Context,
    channel_id: serenity::ChannelId,
) -> Option<serenity::Webhook> {
    if let Ok(webhooks) = channel_id.webhooks(&ctx.http).await {
        return webhooks.into_iter().next();
    } else {
        // either no permission or no webhooks.
        // either way, try to create one.
        if let Ok(new_webhook) = channel_id
            .create_webhook(
                &ctx.http,
                CreateWebhook::new("Moete").audit_log_reason(
                    "Moete requires a webhook to send a message with custom emotes.",
                ),
            )
            .await
        {
            return Some(new_webhook);
        }
    }

    None
}
