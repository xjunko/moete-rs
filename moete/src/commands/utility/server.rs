use crate::{Context, Error};

use moete_discord as discord;
use poise::CreateReply;
use serenity::all::{ChannelType, Mentionable};

/// Replies with the server's information.
#[poise::command(prefix_command, category = "Utility", subcommands("emote", "role"))]
pub async fn server(ctx: Context<'_>) -> Result<(), Error> {
    let embed = {
        let guild = ctx
            .cache()
            .guild(
                ctx.guild_id()
                    .ok_or("This command can only be used in a server.")?,
            )
            .ok_or("This command can only be used in a server.")?;

        let guild_owner = {
            let owner_id = guild.owner_id;
            let owner = ctx
                .cache()
                .user(owner_id)
                .ok_or("Failed to fetch the server owner.")?;
            format!("{}", owner.mention())
        };

        discord::embed::create_embed()
            .title(guild.name.clone())
            .thumbnail(guild.icon_url().unwrap_or(ctx.author().face()))
            .field(
                "Main",
                format!(
                    "Owner: {}\n\
                    Members: {}\n\
                    Emotes: {}\n\
                    Roles: {}",
                    guild_owner,
                    guild.member_count,
                    guild.emojis.len(),
                    guild.roles.len(),
                ),
                false,
            )
            .field(
                "Channels",
                format!(
                    "Category: {}\n\
                    Text: {}\n\
                    Voice: {}",
                    guild
                        .channels
                        .values()
                        .filter(|c| c.kind == ChannelType::Category)
                        .count(),
                    guild
                        .channels
                        .values()
                        .filter(|c| c.kind == ChannelType::Text)
                        .count(),
                    guild
                        .channels
                        .values()
                        .filter(|c| c.kind == ChannelType::Voice)
                        .count(),
                ),
                false,
            )
            .field(
                "Misc",
                format!(
                    "Created at: <t:{}:F>",
                    guild.id.created_at().unix_timestamp()
                ),
                false,
            )
            .field(
                "Emotes",
                format!(
                    "To see a list with all emotes use `{}server emotes`",
                    ctx.prefix()
                ),
                true,
            )
            .field(
                "Roles",
                format!(
                    "To see a list with all roles use `{}server roles`",
                    ctx.prefix()
                ),
                true,
            )
    };

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Replies with a list of all emotes in the server.
#[poise::command(prefix_command, category = "Utility", aliases("emotes"))]
pub async fn emote(ctx: Context<'_>) -> Result<(), Error> {
    let embed = {
        let guild = ctx
            .cache()
            .guild(
                ctx.guild_id()
                    .ok_or("This command can only be used in a server.")?,
            )
            .ok_or("This command can only be used in a server.")?;

        let mut embed = moete_discord::embed::create_embed()
            .title(format!("{} | Emotes", guild.name))
            .thumbnail(guild.icon_url().unwrap_or(ctx.author().face()));

        if guild.emojis.is_empty() {
            embed = embed.description("No emotes found in this server.");
        } else {
            let emotes: Vec<String> = guild.emojis.values().map(|e| e.to_string()).collect();
            embed = embed.description(emotes.join("").chars().take(1900).collect::<String>());
        }

        embed
    };

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

/// Replies with a list of all roles in the server.
#[poise::command(prefix_command, category = "Utility", aliases("roles"))]
pub async fn role(ctx: Context<'_>) -> Result<(), Error> {
    let embed = {
        let guild = ctx
            .cache()
            .guild(
                ctx.guild_id()
                    .ok_or("This command can only be used in a server.")?,
            )
            .ok_or("This command can only be used in a server.")?;

        let mut embed = moete_discord::embed::create_embed()
            .title(format!("{} | Roles", guild.name))
            .thumbnail(guild.icon_url().unwrap_or(ctx.author().face()));

        if guild.roles.is_empty() {
            embed = embed.description("No roles found in this server.");
        } else {
            let roles: Vec<String> = guild
                .roles
                .values()
                .map(|r| format!("{}", r.mention()))
                .collect();
            embed = embed.description(roles.join(", ").chars().take(1900).collect::<String>());
        }

        embed
    };
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
