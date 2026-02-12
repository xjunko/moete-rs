use moete_core::{MoeteContext, MoeteError};

use super::color::is_moete_supported;

/// Strips all the moete color roles from the user.
#[poise::command(
    prefix_command,
    slash_command,
    guild_only,
    category = "Role",
    aliases("clean", "remove")
)]
pub async fn clear(ctx: MoeteContext<'_>) -> Result<(), MoeteError> {
    if is_moete_supported(ctx).await.is_some() {
        let member = ctx
            .guild_id()
            .unwrap()
            .member(ctx.http(), ctx.author().id)
            .await?;

        let roles = {
            let guild = ctx.guild().unwrap();
            guild.roles.clone()
        };

        let mut removed = false;

        for user_role_id in &member.roles {
            if let Some(user_role) = roles.get(user_role_id)
                && user_role.name.starts_with("M#")
            {
                // Looks like a moete color role, remove it.
                member.remove_role(ctx.http(), user_role.id).await?;
                removed = true;
            }
        }

        if removed {
            ctx.say("Your custom color roles have been removed.")
                .await?;
        } else {
            ctx.say("You have no custom color roles to remove.").await?;
        }
    } else {
        ctx.say("Moete's custom color roles are not supported on this server.")
            .await?;
    }
    Ok(())
}
