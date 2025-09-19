use super::ALLOWED;
use crate::core::markov;
use poise::CreateReply;
use rand::Rng;
use serenity::all::{ExecuteWebhook, UserId};
use sqlx::postgres;
use std::sync::Arc;
use tracing::info;

use crate::{Context, Error};
use crate::{builtins, core};

async fn load_data(id: u64, pool: Arc<Option<postgres::PgPool>>) -> Option<String> {
    info!("Loading data for user {}", id);

    if let Some(pool) = pool.as_ref()
        && let Ok(user_data) = markov::get_user(pool, id.try_into().ok()?).await
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

    let user_id = ALLOWED[picked as usize - 1];
    let result = {
        let data = load_data(user_id, pool).await?;
        let text = marukov::Text::new(data);
        let mut rng = rand::rng();
        let res = text.generate(marukov::TextOptions {
            tries: 100,
            min_words: rng.random_range(0..10),
            max_words: rng.random_range(50..100),
        });

        std::mem::drop(text);
        std::mem::drop(rng);

        res
    };

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
    let state: &core::State = ctx.data();

    if let Some(picked) = picked
        && let Some((content, user_id)) = generate(picked, state.pool.clone()).await
    {
        if let Ok(user) = UserId::new(user_id).to_user(ctx.http()).await
            && let Some(webhook) = builtins::discord::webhook::get_or_create_webhook(
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
            if let Some(user) = cache.user(*id) {
                available_users.push(format!("{}. {} | {} messages", n + 1, user.name, -1));
            } else {
                available_users.push(format!("{}. {} | {} messages", n + 1, id, -1));
            }
        }

        let embed = builtins::discord::embed::create_embed()
            .title("Markovify | Main")
            .field("Available", available_users.join("\n"), true);
        ctx.send(CreateReply::default().embed(embed)).await?;
    }

    Ok(())
}
