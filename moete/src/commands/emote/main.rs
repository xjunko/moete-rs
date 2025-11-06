use poise::CreateReply;
use serenity::all::CreateEmbedFooter;

use moete_core::{MoeteContext, MoeteError};

/// Subcommands
use super::search::search;
use super::text::text;

/// Help command for the emote system.
#[allow(clippy::useless_vec)]
#[poise::command(
    prefix_command,
    slash_command,
    category = "Emote",
    aliases("emotes", "e"),
    subcommands("search", "text")
)]
pub async fn emote(ctx: MoeteContext<'_>) -> Result<(), MoeteError> {
    let cmds = vec![
        ["search", "Search emote"],
        ["list", "List all emotes"],
        ["info", "Get EmoteManager info / Emote info"],
        ["add*", "Add an emote to the bot"],
        ["delete*", "Delete an emote from the bot"],
        ["rename*", "Rename an emote in the bot"],
        ["steal*", "Steal an emote from a message"],
        [
            "stealmultiple*",
            "Same thing as `steal` but with multiple message IDs",
        ],
    ]
    .iter()
    .map(|c| format!("`{}`: {}", c[0], c[1]))
    .collect::<Vec<_>>()
    .join("\n");

    let embed = moete_discord::embed::create_embed()
        .title(format!("{} | {}", "Emotes", "Main"))
        .field("Commands", cmds, false)
        .footer(CreateEmbedFooter::new(
            "You need to be whitelisted to use the command with the *.",
        ));
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
