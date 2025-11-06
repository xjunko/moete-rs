use moete_core::{MoeteContext, MoeteError};
use moete_discord::checks::is_owner;

/// Adds an emoji into the bot's database.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Emote",
    check = "is_owner",
    aliases("a")
)]
pub async fn add(
    ctx: MoeteContext<'_>,
    #[description = "Name of the emoji to add"] emoji_name: String,
    #[description = "URL of the emoji image"] emoji_url: String,
) -> Result<(), MoeteError> {
    let data: &moete_core::State = ctx.data();
    let mut emotes = data.emotes.lock().await;

    // check if the emoji already exists
    let matched_emotes = emotes.get_many(&emoji_name);
    if !matched_emotes.is_empty()
        && matched_emotes
            .iter()
            .any(|e| e.name.to_lowercase() == emoji_name.to_lowercase())
    {
        ctx.reply(format!("Emote with name `{}` already exists.", emoji_name))
            .await?;
        return Ok(());
    }

    // ok, whatevs, let's add it.
    if let Some(image_data) = moete_discord::cdn::to_base64(&emoji_url).await {
        match ctx
            .serenity_context()
            .create_application_emoji(
                &emoji_name,
                &format!("data:image/png;base64,{}", image_data),
            )
            .await
        {
            Ok(emoji) => {
                ctx.reply(format!("Added emote: {} {}", emoji, emoji.name))
                    .await?;
                emotes.add_emoji(emoji);
                return Ok(());
            }
            Err(e) => {
                ctx.reply(format!("Failed to add emote: {}", e)).await?;
                return Ok(());
            }
        }
    } else {
        ctx.reply("Failed to fetch image from URL.").await?;
    }

    Ok(())
}
