use poise::CreateReply;
use serenity::all::EditRole;
use serenity::all::Role;

use crate::{Context, Error, serenity};

use super::list::list;

const MOETE_ANCHOR: &str = "=== Moete: Colors ===";

pub fn color_to_hex(col: serenity::Color) -> String {
    format!("#{:06X}", col.0)
}

pub async fn is_moete_supported(ctx: Context<'_>) -> Option<Role> {
    let roles = {
        let guild = ctx.guild()?;
        guild.roles.clone()
    };

    for (_, role) in roles.iter() {
        if role.name.to_lowercase() == MOETE_ANCHOR.to_lowercase() {
            return Some(role.clone());
        }
    }

    None
}

pub async fn get_colour_role_from_server_if_exists_else_make_one(
    ctx: Context<'_>,
    color: serenity::Color,
) -> Option<Role> {
    let (roles, guild_id) = {
        let guild = ctx.guild()?;
        (guild.roles.clone(), guild.id)
    };

    // We verify that we can do custom roles by checking for Moete anchor role
    if let Some(moete_anchor) = is_moete_supported(ctx).await {
        let index = moete_anchor.position;
        let name = format!("M{}", color_to_hex(color));

        // Use existing role if it exists
        for (_, role) in roles.iter() {
            if role.name == name {
                return Some(role.clone());
            }
        }

        // Cooked, make one.
        if let Ok(new_role) = guild_id
            .create_role(
                ctx.http(),
                EditRole::new()
                    .name(name)
                    .colour(color)
                    .position(index)
                    .mentionable(false),
            )
            .await
        {
            return Some(new_role);
        }
    }

    None
}

/// Sets a custom colour role for the user.
#[poise::command(prefix_command, category = "Role", subcommands("list"))]
pub async fn color(
    ctx: Context<'_>,
    #[description = "Color to use for user's role"]
    #[rest]
    optional_color_hex: Option<String>,
) -> Result<(), Error> {
    // If valid colors, we set them
    if let Some(color_str) = optional_color_hex
        && let Some(color) = moete_discord::color::from_string(&color_str)
        && let Some(role) = get_colour_role_from_server_if_exists_else_make_one(ctx, color).await
    {
        let data: &moete_core::State = ctx.data();
        let mut embed = moete_discord::embed::create_embed()
            .title(format!("{} | Self Color Role", data.config.discord.name))
            .color(color)
            .thumbnail(
                ctx.author()
                    .avatar_url()
                    .unwrap_or(ctx.author().default_avatar_url()),
            )
            .field("Info", format!("Hex: {}", color_to_hex(color)), false);

        // Check
        let checking = embed.clone().field("Progress", "```Checking```", false);
        let msg = ctx.send(CreateReply::default().embed(checking)).await?;

        // Can start checking
        let (guild_id, roles) = {
            let Some(guild) = ctx.guild() else {
                ctx.say("This command can only be used in a server").await?;
                return Ok(()); // Something went horribly wrong
            };

            (guild.id, guild.roles.clone())
        };
        let member = guild_id.member(ctx.http(), ctx.author().id).await?;

        for user_role_id in &member.roles {
            if let Some(user_role) = roles.get(user_role_id) {
                // Have we already got the role?
                if user_role.id == role.id {
                    embed = embed.field("Progress", "```Already applied```", false);
                    msg.edit(ctx, CreateReply::default().embed(embed)).await?;
                    return Ok(());
                }

                // Also check for other roles, remove if we find one.
                if user_role.name.starts_with("M#") {
                    // Probably ours, safe to remove.
                    // Requires permission but we assume the bot has it.
                    member.remove_role(ctx.http(), user_role.id).await?;
                }
            }
        }

        // // Erm, add the role.
        member.add_role(ctx.http(), role.id).await?;
        embed = embed.field("Progress", "```Added!```", false);
        msg.edit(ctx, CreateReply::default().embed(embed)).await?;
        return Ok(());
    }

    // If not, then we gotta explain to them how to use the thing.
    let prefix = ctx.prefix();
    let data: &moete_core::State = ctx.data();

    let moete_roles: Vec<Role> = {
        let Some(guild) = ctx.guild() else {
            return Ok(());
        };

        guild
            .roles
            .iter()
            .filter(|(_, e)| e.name.starts_with("M#"))
            .map(|(_, role)| role.clone())
            .collect()
    };

    let embed = moete_discord::embed::create_embed()
            .title(format!(
                "{} | Self Color Role [Help]",
                data.config.discord.name
            )).field(
                "Example",
                format!("**With Hex Value**:```{}color #ADD8E6```\nRefer to the picture below for tutorial.\n", prefix),
                false,
            ).field("Info", format!("Created Roles: `{}`", moete_roles.len()), false).field("Tutorial", "_ _", false).image("https://cdn.discordapp.com/attachments/1390250982855938172/1392011529599193108/weHwReg.png");

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
