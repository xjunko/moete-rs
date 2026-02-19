use std::sync::Arc;

use moete_core::{MoeteContext, MoeteError};
use poise::CreateReply;
use rand::Rng;
use serenity::all::{ExecuteWebhook, UserId};
use tracing::info;

use super::ALLOWED;

async fn load_data(id: u64, database: &moete_database::Database) -> Option<String> {
    info!("Loading data for user {}", id);

    if let Ok(user_data) = database.get_user(id.try_into().ok()?).await
        && let Some(data) = user_data
    {
        info!("Loaded {} messages for user {}", data.messages.len(), id);
        return Some(
            data.messages
                .iter()
                .map(|m| m.content.as_str())
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }

    None
}

pub async fn generate(
    picked: i32,
    starter: Option<String>,
    database: &moete_database::Database,
) -> Option<(Option<String>, u64)> {
    if picked <= 0 || picked > ALLOWED.len() as i32 {
        return None;
    }

    let outer = std::time::Instant::now();
    let user_id = ALLOWED[picked as usize - 1];
    let result: Option<String> = {
        // load data
        let start = std::time::Instant::now();
        let data = load_data(user_id, database).await?;
        if data.is_empty() {
            return None;
        }

        info!("Loaded data in {:?}", start.elapsed());

        // build model
        let start = std::time::Instant::now();
        let text = marukov::Text::new(data);
        info!("Built model in {:?}", start.elapsed());

        // generate text
        let start = std::time::Instant::now();
        let mut rng = rand::rng();
        let options = marukov::TextOptions {
            tries: 999,
            min_words: rng.random_range(0..10),
            max_words: rng.random_range(50..100),
            ..Default::default()
        };

        let res = if let Some(starter) = starter {
            // HACK: the user might think that the starter is multiple words.
            // so we just take the last word as the actual starter.
            // if failed, we fallback to normal generation.
            let (others, last_word) = {
                let mut parts = starter.split_whitespace();
                let last = parts.next_back()?.to_string();
                let rest = parts.collect::<Vec<_>>().join(" ");
                (rest, last)
            };

            info!("Using starter word: {}", last_word);

            if let Some(generated) = text.generate_with_start(&last_word, options.clone()) {
                Some(format!("{} {}", others, generated))
            } else {
                text.generate(options)
            }
        } else {
            text.generate(options)
        };

        info!("Generated text in {:?}", start.elapsed());

        std::mem::drop(text);
        std::mem::drop(rng);

        res
    };

    info!("Total time: {:?}", outer.elapsed());

    // text generation uses a lot of memory, trim the memory here.
    moete_core::memory::trim_memory();

    Some((result, user_id))
}

/// Generates a random text based on the user's messages.
#[poise::command(prefix_command, slash_command, category = "Markov", aliases("deep"))]
pub async fn markov(
    ctx: MoeteContext<'_>,
    #[description = "User to generate text for"] picked: Option<i32>,
    #[description = "Starting text"]
    #[rest]
    starter: Option<String>,
) -> Result<(), MoeteError> {
    let state: &moete_core::State = ctx.data();

    if let Some(database) = &state.database
        && let Some(picked) = picked
        && let Some((content, user_id)) = generate(picked, starter, database).await
    {
        // handle empty content
        let content =
            content.unwrap_or("Generation failed, must've been insufficient data.".to_string());

        if let Ok(user) = UserId::new(user_id).to_user(ctx.http()).await
            && let Some(webhook) = moete_discord::webhook::get_or_create_webhook(
                ctx.serenity_context(),
                ctx.channel_id(),
            )
            .await
        {
            let _ = webhook
                .execute(
                    ctx.serenity_context(),
                    true,
                    ExecuteWebhook::new()
                        .username(user.display_name())
                        .avatar_url(user.face())
                        .content(content),
                )
                .await;
        }
    } else {
        // Show everyone's stats on error.
        let mut available_users = Vec::new();
        let cache = Arc::clone(&ctx.serenity_context().cache);
        for (n, id) in ALLOWED.iter().enumerate() {
            // Get user count
            let count = if let Some(database) = &state.database {
                match database.get_user_count(*id as i64).await {
                    Ok(Some(c)) => c,
                    _ => 0,
                }
            } else {
                0
            };

            // Gets user from discord cache
            if let Some(user) = cache.user(*id) {
                available_users.push(format!("{}. {} | {} messages", n + 1, user.name, count));
            } else {
                available_users.push(format!("{}. {} | {} messages", n + 1, id, count));
            }
        }

        let embed = moete_discord::embed::create_embed()
            .title("Markovify | Main")
            .field("Available", available_users.join("\n"), true);
        ctx.send(CreateReply::default().embed(embed).reply(true))
            .await?;
    }

    Ok(())
}
