use crate::{Context, Error};

/// Replies with a nhentai link for a given code.
#[poise::command(prefix_command, category = "Utility")]
pub async fn nhentai(
    ctx: Context<'_>,
    #[description = "Doujin code"]
    #[rest]
    id: String,
) -> Result<(), Error> {
    ctx.reply(format!("https://nhentai.net/g/{}", id)).await?;
    Ok(())
}
