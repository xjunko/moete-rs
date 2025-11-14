use moete_core::{MoeteContext, MoeteError};

/// Search for emotes matching a query.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Emote",
    aliases("list", "ls", "es", "se", "el")
)]
pub async fn search(
    ctx: MoeteContext<'_>,
    #[description = "Query for the emote"]
    #[rest]
    query: Option<String>,
) -> Result<(), MoeteError> {
    let query_or_all = query.unwrap_or_default();
    let data: &moete_core::State = ctx.data();
    let emotes_page_estimated = data.emotes.lock().await.get_many(&query_or_all).len() / 20 + 1;
    let emotes = data
        .emotes
        .lock()
        .await
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
                emotes_page_estimated,
                query_display,
                page
            )
        })
        .collect::<Vec<_>>();

    if emotes.is_empty() {
        ctx.reply(format!("No emotes found for query: `{}`", query_or_all))
            .await?;
    } else {
        moete_discord::paginate::paginate(ctx, emotes).await?;
    }

    Ok(())
}
