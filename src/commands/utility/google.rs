use crate::{Context, Error};

/// Replies with a Google search link for the given query.
#[poise::command(prefix_command)]
pub async fn google(
    ctx: Context<'_>,
    #[description = "Query to search"]
    #[rest]
    query: String,
) -> Result<(), Error> {
    ctx.say(format!(
        "https://www.google.com/search?q={}",
        query.replace(" ", "+")
    ))
    .await?;
    Ok(())
}
