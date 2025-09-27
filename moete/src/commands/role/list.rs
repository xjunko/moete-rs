use poise::CreateReply;
use serenity::all::Role;

use super::color::color_to_hex;

use crate::serenity;
use moete_core::{MoeteContext, MoeteError};

/// Lists all the custom color roles in this server.
#[poise::command(prefix_command, slash_command, category = "Role")]
pub async fn list(ctx: MoeteContext<'_>) -> Result<(), MoeteError> {
    let roles: Vec<Role> = {
        let Some(guild) = ctx.guild() else {
            ctx.say("This command can only be used in a server.")
                .await?;
            return Ok(());
        };

        guild
            .roles
            .values()
            .filter(|r| r.name.starts_with("M#"))
            .cloned()
            .collect()
    };

    if roles.is_empty() {
        ctx.say("No custom color roles found in this server.")
            .await?;
        return Ok(());
    }

    let data: &moete_core::State = ctx.data();
    let mut embed = moete_discord::embed::create_embed()
        .title(format!("{} | Color Roles", data.config.discord.name))
        .description("List of all self-assignable color roles in this server.")
        .thumbnail(ctx.author().face());

    let roles_len = roles.len();
    for role in roles {
        embed = embed.field(
            role.name,
            format!("Hex: {}", color_to_hex(role.colour)),
            false,
        )
    }

    embed = embed.field(
        "Total Roles",
        format!("{} self-assignable color roles round.", roles_len),
        false,
    );

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
