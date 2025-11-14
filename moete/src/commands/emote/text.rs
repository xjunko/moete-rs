use serenity::all::ExecuteWebhook;
use tracing::error;

use moete_core::{MoeteContext, MoeteError};

/// Sends a message
async fn send_message(
    ctx: MoeteContext<'_>,
    who: serenity::all::User,
    msg: String,
) -> Result<(), MoeteError> {
    if let poise::Context::Prefix(prefix_ctx) = ctx
        && let Err(e) = prefix_ctx
            .msg
            .delete(&ctx.serenity_context().http.clone())
            .await
    {
        error!("Failed to delete message: {:?}", e);
    }

    let data: &moete_core::State = ctx.data();
    let msg = data.emotes.lock().await.text(&msg);

    if let Some(webhook) =
        moete_discord::webhook::get_or_create_webhook(ctx.serenity_context(), ctx.channel_id())
            .await
    {
        let user = who;

        if let Err(e) = webhook
            .execute(
                ctx.serenity_context().http.clone(),
                true,
                ExecuteWebhook::new()
                    .content(msg.clone())
                    .avatar_url(user.face())
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

/// Send a message through Moete's emote system.
#[poise::command(prefix_command, slash_command, category = "Emote", aliases("txt", "t"))]
pub async fn text(
    ctx: MoeteContext<'_>,
    #[description = "Text to send"]
    #[rest]
    msg: String,
) -> Result<(), MoeteError> {
    let user = moete_discord::user::get_member_or_user(&ctx).await?;
    send_message(ctx, user, msg).await?;
    Ok(())
}

/// Sends a message as another person through Moete's emote system.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Emote",
    aliases("ta", "ft", "textas")
)]
pub async fn text_as(
    ctx: MoeteContext<'_>,
    #[description = "User to send as"] user: serenity::all::User,
    #[description = "Text to send"]
    #[rest]
    msg: String,
) -> Result<(), MoeteError> {
    send_message(ctx, user, msg).await?;
    Ok(())
}
