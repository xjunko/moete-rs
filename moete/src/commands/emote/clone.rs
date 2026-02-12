use moete_core::{MoeteContext, MoeteError};
use moete_discord::checks::is_owner;
use poise::CreateReply;

use super::regexes::RE_EMOTE;
use crate::serenity;

/// Clones an emoji from a message, message_id, reference, url or literal emote.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Utility",
    check = "is_owner",
    aliases("steal")
)]
pub async fn clone(
    ctx: MoeteContext<'_>,
    #[description = "Message URL, ID, Reply or Emote"] message_or_emote_opt: Option<String>,
) -> Result<(), MoeteError> {
    // only support prefix command for now
    if let poise::Context::Prefix(prefix_ctx) = ctx {
        let mut message_id: Option<serenity::MessageId> = None;
        let mut channel_id: Option<serenity::ChannelId> = None;

        let message_or_emote = message_or_emote_opt.unwrap_or_default();

        if message_or_emote.starts_with("https://") {
            let items: Vec<&str> = message_or_emote.split('/').collect();
            channel_id = serenity::ChannelId::new(
                items[items.len() - 2].parse().expect("Invalid channel ID"),
            )
            .into();
            message_id = serenity::MessageId::new(
                items[items.len() - 1].parse().expect("Invalid message ID"),
            )
            .into();
        } else if message_or_emote.trim().parse::<f64>().is_ok() {
            channel_id = ctx.channel_id().into();
            message_id = serenity::MessageId::new(message_or_emote.trim().parse().unwrap()).into();
        } else if prefix_ctx.msg.referenced_message.is_some() {
            channel_id = ctx.channel_id().into();
            message_id = prefix_ctx.msg.referenced_message.as_ref().map(|m| m.id);
        } else if message_or_emote.trim().is_empty() {
            ctx.reply("Supports: Direct URL, Reply, Emote").await?;
            return Ok(());
        }

        let mut content: String = message_or_emote.clone();
        if let (Some(message_id), Some(channel_id)) = (message_id, channel_id) {
            let channel = channel_id.to_channel(&ctx.http()).await?;
            if let serenity::Channel::Guild(guild_channel) = channel {
                let message = guild_channel.message(&ctx.http(), message_id).await?;
                content = message.content.clone();
            }
        }

        if !RE_EMOTE.is_match(&content) {
            ctx.reply("No emote found in the message.").await?;
            return Ok(());
        }

        let emotes = RE_EMOTE
            .find_iter(&content)
            .map(|mat| mat.as_str().to_string())
            .collect::<Vec<String>>();

        // add emotes
        let data: &moete_core::State = ctx.data();
        let mut emotes_manager = data.emotes.lock().await;

        let progress = ctx.reply("Loading!").await?;
        let mut success = 0;
        let mut failed = 0;

        for (i, emote) in emotes.iter().enumerate() {
            let cleaned = emote.replace("<", "").replace(">", "");
            let items = cleaned.split(':').collect::<Vec<&str>>();

            let animated = items[0] == "a";
            let name = items[1];
            let id = items[2];
            let link = {
                if animated {
                    format!("https://cdn.discordapp.com/emojis/{}.gif", id)
                } else {
                    format!("https://cdn.discordapp.com/emojis/{}.png", id)
                }
            };

            if let Some(image_data) = moete_discord::cdn::to_base64(&link).await {
                match ctx
                    .serenity_context()
                    .create_application_emoji(
                        name,
                        &format!("data:image/png;base64,{}", image_data),
                    )
                    .await
                {
                    Ok(emoji) => {
                        success += 1;
                        emotes_manager.add_emoji(emoji);
                    },
                    Err(_) => {
                        failed += 1;
                    },
                }
            }

            // edit progress
            progress
                .edit(
                    ctx,
                    CreateReply::default()
                        .content(format!(
                            "Progress: {}/{} | Success: {} | Failed: {}",
                            i + 1,
                            emotes.len(),
                            success,
                            failed
                        ))
                        .reply(true),
                )
                .await?;
        }
    }
    Ok(())
}
