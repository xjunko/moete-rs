use crate::serenity;
use moete_core::{MoeteContext, MoeteError};

pub async fn get_member_or_user(ctx: &MoeteContext<'_>) -> Result<serenity::User, MoeteError> {
    let mut user = ctx.author().clone();
    if let Some(member) = ctx.author_member().await {
        user = member.user.clone();
    }
    Ok(user)
}
