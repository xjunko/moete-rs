use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

use crate::serenity;
use serenity::all::Role;
use serenity::all::{GuildId, RoleId};

use tracing::error;

use super::color::MOETE_ANCHOR;
use moete_core::MoeteError;

/// Bit like the one from colors.rs but for serenity::Context
async fn is_moete_supported(
    ctx: serenity::Context,
    message: &serenity::Message,
) -> Option<Vec<Role>> {
    if let Some(guild_id) = message.guild_id
        && let Some(guild) = guild_id.to_guild_cached(&ctx.cache)
    {
        let roles = guild.roles.clone();

        for (_, role) in roles.iter() {
            if role.name.to_lowercase() == MOETE_ANCHOR.to_lowercase() {
                return Some(roles.values().cloned().collect());
            }
        }
    }

    None
}

/// Check if any of the members has the role assigned
async fn is_role_in_use(
    ctx: &serenity::Context,
    guild_id: GuildId,
    role_id: RoleId,
) -> Result<bool, MoeteError> {
    let members = guild_id.members(&ctx.http, None, None).await?;

    for member in members.iter() {
        if member.roles.contains(&role_id) {
            return Ok(true);
        }
    }

    Ok(false)
}

static GUILD_COOLDOWNS: Lazy<Mutex<HashMap<GuildId, Instant>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Deletes all the unused color roles created by Moete.
pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    _: &moete_core::State,
) -> Result<(), MoeteError> {
    let guild_id = match message.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    {
        let mut cooldowns = GUILD_COOLDOWNS.lock().await;
        if let Some(last_run) = cooldowns.get(&guild_id)
            && last_run.elapsed() < Duration::from_secs(30 * 60)
        {
            // 30 minutes cooldown
            return Ok(());
        }

        // not in cooldown, set new timestamp
        cooldowns.insert(guild_id, Instant::now());
    }

    // delete unused color roles
    if let Some(roles) = is_moete_supported(ctx.clone(), message).await {
        for role in roles {
            if !role.managed
                && role.name.starts_with("M#")
                && !is_role_in_use(ctx, guild_id, role.id).await?
            {
                // most likely we made it, can be safely delete.
                if let Err(err) = guild_id.delete_role(&ctx.http, role.id).await {
                    error!("Failed to delete role {}: {:?}", role.name, err);
                }
            }
        }
        return Ok(());
    }
    Ok(())
}
