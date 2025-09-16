use crate::builtins::discord;
use crate::core;
use crate::{Context, Error};

/// Search for emotes matching a query.
#[poise::command(prefix_command, category = "Emote", aliases("list"))]
pub async fn search(
    ctx: Context<'_>,
    #[description = "Query for the emote"]
    #[rest]
    query: Option<String>,
) -> Result<(), Error> {
    let query_or_all = query.unwrap_or_default();
    let data: &core::State = ctx.data();
    let emotes = data
        .emotes
        .get_many(&query_or_all)
        .chunks(20)
        .enumerate()
        .map(|(i, chunk)| {
            let page = chunk
                .iter()
                .map(|e| format!("{} {}", e, e.name))
                .collect::<Vec<_>>()
                .join("\n");

            let query_display = if query_or_all.is_empty() {
                "All"
            } else {
                &query_or_all
            };

            format!(
                "**Emote | List [Page: {}/{}]**\n**Query**: `{}`\n{}",
                i + 1,
                chunk.len(),
                query_display,
                page
            )
        })
        .collect::<Vec<_>>();

    discord::paginate::paginate(ctx, emotes).await?;

    Ok(())
}
