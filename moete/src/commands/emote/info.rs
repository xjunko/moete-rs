use poise::CreateReply;
use serenity::all::Emoji;

use moete_core::{MoeteContext, MoeteError};

/// Get info about an emote.
#[allow(deprecated)]
#[poise::command(
    prefix_command,
    slash_command,
    category = "Utility",
    aliases("ei", "emoteinfo")
)]
pub async fn info(
    ctx: MoeteContext<'_>,
    #[description = "Emote to get info about"]
    #[rest]
    emote: Emoji,
) -> Result<(), MoeteError> {
    let mut embed = moete_discord::embed::create_embed().title("Emotes | Info (Emote)");

    // main
    embed = embed.field(
        "Main",
        {
            format!(
                "Name: {}\nID: {}\nFrom: {}",
                emote.name,
                emote.id,
                emote
                    .find_guild_id(ctx.cache())
                    .map_or("Unknown".to_string(), |g| g.to_string()),
            )
        },
        false,
    );

    // metadata
    embed = embed.field(
        "Metadata",
        {
            format!(
                "Animated: {}\nManaged: {}\nAvailable: {}",
                emote.animated, emote.managed, emote.available
            )
        },
        false,
    );

    // picture
    embed = embed.thumbnail(emote.url());

    let mut reply = CreateReply::default().embed(embed).reply(true);
    reply.content = format!("**Link**: {}", emote.url()).into();
    ctx.send(reply).await?;

    Ok(())
}
