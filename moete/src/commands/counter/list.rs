use moete_core::{MoeteContext, MoeteError};
use serenity::all::UserId;

use super::listener::WORDS;
use crate::serenity;

const LIMIT: usize = 20;

/// Returns lists of words being counted.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Utility",
    aliases("wordcount", "wc")
)]
pub async fn count(ctx: MoeteContext<'_>) -> Result<(), MoeteError> {
    let data: &moete_core::State = ctx.data();
    let mut embeds: Vec<serenity::CreateEmbed> = Vec::new();

    for (word, _) in WORDS.iter() {
        let mut embed = moete_discord::embed::create_embed()
            .title(format!(
                "{} | {}-word counter",
                data.config.discord.name, word
            ))
            .thumbnail(ctx.author().face());

        if let Some(pool) = data.pool.as_ref() {
            let counters = moete_database::counter::get_counters(pool, word).await?;

            // user info
            let user_count = counters
                .iter()
                .find(|c| c.user_id == i64::from(ctx.author().id))
                .map(|c| c.count)
                .unwrap_or(0);
            embed = embed.field(
                "Info",
                format!(
                    "You have said `{}` and it's variations for `{}` times!",
                    word, user_count
                ),
                false,
            );

            // lb
            let mut content: Vec<String> = Vec::new();
            for (i, c) in counters.iter().enumerate() {
                if i >= LIMIT {
                    break;
                }

                let user = UserId::new(c.user_id.try_into().unwrap());
                let username = if let Some(user) = ctx.cache().user(user) {
                    user.name.clone()
                } else {
                    // ctx.http().get_user(user).await.unwrap().name
                    // ^ slow
                    format!("Unknown User ({})", c.user_id)
                };

                content.push(format!(
                    "{}. {}: {} {}",
                    i + 1,
                    username,
                    c.count,
                    if c.count == 1 { "time" } else { "times" }
                ));
            }

            embed = embed.field(
                "Leaderboard",
                format!("```{}```", content.join("\n")),
                false,
            );
            embeds.push(embed);
        }
    }

    if embeds.is_empty() {
        ctx.say("No counters found.").await?;
    } else {
        moete_discord::paginate::paginate_embed(ctx, embeds).await?;
    }

    Ok(())
}
