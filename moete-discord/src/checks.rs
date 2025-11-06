use moete_core::{MoeteContext, MoeteError};

/// Check if the user is an owner of the bot.
pub async fn is_owner(ctx: MoeteContext<'_>) -> Result<bool, MoeteError> {
    let owners = ctx.data().config.moete.owners;
    let user_id = ctx.author().id;
    Ok(owners.contains(&user_id.get()))
}
