use crate::core;
use crate::{Context, Error};

/// Send a message through Moete's emote system.
#[poise::command(prefix_command, category = "Text")]
pub async fn text(
    ctx: Context<'_>,
    #[description = "Text to send"]
    #[rest]
    msg: String,
) -> Result<(), Error> {
    let data: &core::State = ctx.data();
    ctx.reply(data.emotes.text(&msg)).await?;
    Ok(())
}
