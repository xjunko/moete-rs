use moete_core::{MoeteContext, MoeteError};

/// Replies with a Google search link for the given query.
#[poise::command(prefix_command, slash_command, category = "Utility")]
pub async fn google(
    ctx: MoeteContext<'_>,
    #[description = "Query to search"]
    #[rest]
    query: String,
) -> Result<(), MoeteError> {
    ctx.reply(format!(
        "https://www.google.com/search?q={}",
        query.replace(" ", "+")
    ))
    .await?;
    Ok(())
}
