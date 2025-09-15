use crate::{Context, Error, serenity};

pub async fn get_member_or_user(ctx: &Context<'_>) -> Result<serenity::User, Error> {
    let mut user = ctx.author().clone();
    if let Some(member) = ctx.author_member().await {
        user = member.user.clone();
    }
    Ok(user)
}
