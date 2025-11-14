use poise::CreateReply;

use moete_core::{MoeteContext, MoeteError};
use moete_discord as discord;

/// Sends an invite link to the bot.
#[poise::command(prefix_command, slash_command, category = "Utility")]
pub async fn invite(ctx: MoeteContext<'_>) -> Result<(), MoeteError> {
    let bot_user = ctx.serenity_context().http.get_current_user().await?;
    let embed = discord::embed::create_embed()
        .title(format!("{} | Invite Link", ctx.data().config.discord.name))
        .description(format!("Click the link below to invite me to your server!\n\n\
        [Invite Me!](https://discord.com/api/oauth2/authorize?client_id={}&permissions=3758615632&scope=bot)", bot_user.id))
        .thumbnail(bot_user.face());
    ctx.send(CreateReply::default().embed(embed).reply(true))
        .await?;
    Ok(())
}
