use rand::Rng;
use std::{collections::HashMap, time::Duration};

use crate::serenity;
use once_cell::sync::Lazy;
use serenity::all::CacheHttp;
use serenity::all::GuildId;
use tokio::{sync::Mutex, time::Instant};

use moete_core::MoeteError;

static LAST_UPDATED: Lazy<Mutex<Instant>> =
    Lazy::new(|| Mutex::new(Instant::now() - Duration::from_secs(128)));
static SUPPORTED_GUILDS: Lazy<Mutex<HashMap<GuildId, Vec<moete_core::shortcut::Shortcut>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

/*
    FIXME: Redo this entire file, as it's really hacky and ugly.

    the idea was that i don't want to fetch guild_ids from the database on every message as that is expensive.
    so instead, we have a cache that updates every now and then that stores the guild_ids that have shortcuts.

    then on every message, we check if the message.guild_id() is in the cache, if so, we can safely assume that there are
    shortcuts for this guild, and execute accordingly.

    the problem comes when shortcuts are added/removed, as the cache will be stale until the next update.
    this can lead to situations where a user adds a shortcut, but it doesn't work until the cache updates.
    similarly, if a shortcut is removed, it may still trigger until the cache updates.
*/

pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &moete_core::State,
) -> Result<(), MoeteError> {
    // FIXME: everything here is really ugly and hacky

    // updates every minute
    {
        let mut last_updated = LAST_UPDATED.lock().await;
        if last_updated.elapsed().as_secs() >= 60 {
            let mut supported_guilds = SUPPORTED_GUILDS.lock().await;
            supported_guilds.clear();

            if let Some(pool) = data.pool.as_ref() {
                for guild in moete_core::shortcut::get_guild_ids(pool).await? {
                    let shortcuts =
                        moete_core::shortcut::get_all_shortcuts_for_guild(pool, guild).await?;
                    supported_guilds.insert(GuildId::new(guild as u64), shortcuts);
                }
            }

            *last_updated = Instant::now();
        }
    }

    // check if this guild is supported.
    let supported_guilds = SUPPORTED_GUILDS.lock().await;
    if let Some(guild_id) = message.guild_id
        && supported_guilds.contains_key(&guild_id)
        && let Some(shortcuts) = supported_guilds.get(&guild_id)
    {
        for shortcut in shortcuts {
            let (main_prefix, additional_prefixes) = data.config.get_prefixes();

            // check if user is running the shortcut
            if ["/", main_prefix.as_str()]
                .iter()
                .chain(additional_prefixes.iter().map(|s| match s {
                    poise::Prefix::Literal(s) => s,
                    _ => panic!("Expecting Literal prefixes, received Regex"),
                }))
                .any(|prefix| {
                    message.content.to_lowercase().starts_with(&format!(
                        "{}{}",
                        prefix,
                        shortcut.trigger.to_lowercase()
                    ))
                })
            {
                // got one, pick
                let responses: Vec<String> = shortcut
                    .response
                    .clone()
                    .split(",")
                    .map(|s| s.to_string())
                    .collect();

                let response = responses
                    .get(rand::rng().random_range(0..responses.len()))
                    .unwrap()
                    .clone();

                message.reply(ctx.http(), response).await?;
            }
        }
    }

    Ok(())
}
