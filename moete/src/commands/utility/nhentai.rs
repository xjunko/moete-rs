use moete_core::{MoeteContext, MoeteError};

/// Replies with a nhentai link for a given code.
#[poise::command(prefix_command, category = "Utility")]
pub async fn nhentai(
    ctx: MoeteContext<'_>,
    #[description = "Doujin code"]
    #[rest]
    id: String,
) -> Result<(), MoeteError> {
    ctx.reply(format!("https://nhentai.net/g/{}", id)).await?;
    Ok(())
}
