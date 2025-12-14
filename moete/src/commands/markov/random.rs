use std::collections::HashMap;

use moete_core::MoeteError;
use once_cell::sync::Lazy;
use rand::{random, rng, seq::IndexedRandom};
use serenity::all::{ChannelId, ExecuteWebhook};
use tokio::sync::Mutex;

use super::{ALLOWED, text::generate};
use crate::serenity;

const RATE: f32 = 0.02; // 2% 
const PER_MESSAGE: i32 = 20; // seems reasonable.
static CHANNEL_COUNTER: Lazy<Mutex<HashMap<ChannelId, i32>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/// Generates a random message whenever possible.
/// Loosely based on the old impl:
/// https://github.com/xjunko/moete/blob/main/moete/commands/other/markov.py#L452
pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &moete_core::State,
) -> Result<(), MoeteError> {
    if message.author.bot {
        return Ok(());
    }

    // we only run the stuff below if we pass the rate check.
    {
        let mut counts = CHANNEL_COUNTER.lock().await;
        let counter = counts.entry(message.channel_id).or_insert(0);
        *counter += 1;

        if *counter < PER_MESSAGE {
            return Ok(());
        }
        *counter = 0; // reset after hitting the threshold.
    }

    if random::<f32>() < RATE
        && let Some(guild_id) = message.guild_id
    {
        let mut avail_members = Vec::new();

        // check if any of our whitelisted member are in this server.
        for member in guild_id.members(ctx.http.clone(), None, None).await? {
            if ALLOWED.contains(&member.user.id.into()) {
                avail_members.push(member.user.id);
            }
        }

        // no available members, skip.
        if avail_members.is_empty() {
            return Ok(());
        }

        // start generating based on the available members.
        let target_member = {
            let mut rng = rng();
            avail_members
                .choose(&mut rng)
                .unwrap_or(avail_members.first().unwrap()) // the unwraps are safe due to the is_empty check above.
        };

        let picked_option = ALLOWED
            .iter()
            .position(|id| *id == target_member.get())
            .map(|idx| idx as i32 + 1); // offset by one since generate uses 1-based index.

        // safe to assume we can generate now.
        if let Some((content, _)) = generate(picked_option.unwrap_or(1), data.pool.clone()).await
            && let Ok(user) = target_member.to_user(ctx.http.clone()).await
            && let Some(webhook) =
                moete_discord::webhook::get_or_create_webhook(ctx, message.channel_id).await
        {
            let _ = webhook
                .execute(
                    ctx,
                    true,
                    ExecuteWebhook::new()
                        .username(user.display_name())
                        .avatar_url(user.face())
                        .content(content),
                )
                .await;
        }
    }

    Ok(())
}
