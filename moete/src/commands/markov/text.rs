use poise::CreateReply;
use rand::Rng;
use serenity::all::{ExecuteWebhook, UserId};
use sqlx::postgres;
use std::sync::Arc;
use tracing::info;

use super::ALLOWED;

use crate::{Context, Error};

async fn load_data(id: u64, pool: Arc<Option<postgres::PgPool>>) -> Option<String> {
    info!("Loading data for user {}", id);

    if let Some(pool) = pool.as_ref()
        && let Ok(user_data) = moete_core::markov::get_user(pool, id.try_into().ok()?).await
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

async fn generate(picked: i32, pool: Arc<Option<postgres::PgPool>>) -> Option<(String, u64)> {
    if picked <= 0 || picked > ALLOWED.len() as i32 {
        return None;
    }

    let outer = std::time::Instant::now();
    let user_id = ALLOWED[picked as usize - 1];
    let result: String = {
        // load data
        let start = std::time::Instant::now();
        let data = load_data(user_id, pool).await?;
        if data.is_empty() {
            return Some(("Not enough data!".to_string(), user_id));
        }
        info!("Loaded data in {:?}", start.elapsed());

        // build model
        let start = std::time::Instant::now();
        let text = marukov::Text::new(data);
        info!("Built model in {:?}", start.elapsed());

        // generate text
        let start = std::time::Instant::now();
        let mut rng = rand::rng();
        let res = text.generate(marukov::TextOptions {
            tries: 999,
            min_words: rng.random_range(0..10),
            max_words: rng.random_range(50..100),
        });
        info!("Generated text in {:?}", start.elapsed());

        std::mem::drop(text);
        std::mem::drop(rng);

        res
    };
    info!("Total time: {:?}", outer.elapsed());

    // text generation uses a lot of memory, trim the memory here.
    unsafe {
        libc::malloc_trim(0);
    }

    Some((result.clone(), user_id))
}

/// Generates a random text based on the user's messages.
#[poise::command(prefix_command, category = "Markov", aliases("deep"))]
pub async fn markov(
    ctx: Context<'_>,
    #[description = "User to generate text for"] picked: Option<i32>,
) -> Result<(), Error> {
    let state: &moete_core::State = ctx.data();

    if let Some(picked) = picked
        && let Some((content, user_id)) = generate(picked, state.pool.clone()).await
    {
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
                        .avatar_url(user.avatar_url().unwrap_or(user.default_avatar_url()))
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
            let count = if let Some(pool) = state.pool.as_ref() {
                match moete_core::markov::get_user_count(pool, *id as i64).await {
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
        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
