use serenity::all::ExecuteWebhook;
use tracing::error;

use crate::{Context, Error};

/// Send a message through Moete's emote system.
#[poise::command(prefix_command, category = "Emote", aliases("txt", "t"))]
pub async fn text(
    ctx: Context<'_>,
    #[description = "Text to send"]
    #[rest]
    msg: String,
) -> Result<(), Error> {
    if let poise::Context::Prefix(prefix_ctx) = ctx
        && let Err(e) = prefix_ctx
            .msg
            .delete(&ctx.serenity_context().http.clone())
            .await
    {
        error!("Failed to delete message: {:?}", e);
    }

    let data: &moete_core::State = ctx.data();
    let msg = data.emotes.text(&msg);

    if let Some(webhook) =
        moete_discord::webhook::get_or_create_webhook(ctx.serenity_context(), ctx.channel_id())
            .await
    {
        let user = moete_discord::user::get_member_or_user(&ctx).await?;

        if let Err(e) = webhook
            .execute(
                ctx.serenity_context().http.clone(),
                true,
                ExecuteWebhook::new()
                    .content(msg.clone())
                    .avatar_url(user.avatar_url().unwrap_or(user.default_avatar_url()))
                    .username(user.display_name()),
            )
            .await
        {
            error!("Failed to execute webhook: {:?}", e);
        } else {
            return Ok(());
        }
    }

    // Fallback to normal message if webhook fails
    ctx.say(format!("{} - {}", msg.clone(), ctx.author().display_name()))
        .await?;

    Ok(())
}
