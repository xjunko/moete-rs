use moete_core::{MoeteContext, MoeteError};
use moete_discord::checks::is_owner;

/// Deletes an emoji from the bot's database.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Emote",
    check = "is_owner",
    aliases("delete", "del", "rm")
)]
pub async fn remove(ctx: MoeteContext<'_>, emote_name: String) -> Result<(), MoeteError> {
    // now we have to resolve this to an actual emoji.
    // sometimes the user sends the actual emoji, sometimes just the name.
    let data: &moete_core::State = ctx.data();
    let mut emotes = data.emotes.lock().await;

    // target emote
    let mut target_emote: u64 = 0;

    // check if its in emoji format
    // <:name:id> or <a:name:id>
    if emote_name.starts_with("<") {
        // most likely an emoji
        let emote_id_opt = {
            let parts = emote_name
                .split(":")
                .last()
                .expect("Invalid emoji format")
                .split(">")
                .collect::<Vec<_>>();
            parts[0].parse::<u64>().ok()
        };

        if emote_id_opt.is_none() {
            ctx.reply("Invalid emoji format.").await?;
            return Ok(());
        }

        let emote_id = emote_id_opt.expect("Expecting emote id");

        // check if we can own this emote.
        if !emotes.is_our_emoji(emote_id) {
            ctx.reply("Not managed by Moete, can't delete.").await?;
            return Ok(());
        }

        target_emote = emote_id;
    }

    // not in emoji format, try to search by name.
    if target_emote == 0
        && let Some(emote) = emotes.get(&emote_name)
    {
        // check if we own this emote.
        if !emotes.is_our_emoji(emote.id.get()) {
            ctx.reply("Not managed by Moete, can't delete.").await?;
            return Ok(());
        }

        target_emote = emote.id.get();
    }

    if target_emote != 0 {
        match ctx
            .serenity_context()
            .delete_application_emoji(target_emote.into())
            .await
        {
            Ok(_) => {
                if emotes.remove_emoji_by_id(target_emote).is_some() {
                    ctx.reply("Emote deleted successfully.").await?;
                }
            }

            Err(e) => {
                ctx.reply(format!("Failed to delete emote: {}", e)).await?;
            }
        }

        return Ok(());
    }

    ctx.reply("Emote not found.").await?;
    Ok(())
}
