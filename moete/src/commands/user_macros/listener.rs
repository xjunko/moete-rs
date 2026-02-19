use crate::serenity;
use rand::Rng;
use serenity::all::CacheHttp;

use moete_core::MoeteError;

pub async fn on_message(
    ctx: &serenity::Context,
    message: &serenity::Message,
    data: &moete_core::State,
) -> Result<(), MoeteError> {
    if message.author.bot {
        return Ok(());
    }

    let guild_id = match message.guild_id {
        Some(id) => id,
        None => return Ok(()),
    };

    let shortcuts = if let Some(cached) = data.shortcut_cache.get(guild_id.into()) {
        cached
    } else {
        let pool = match data.pool.as_ref() {
            Some(p) => p,
            None => return Ok(()),
        };
        let rows =
            moete_database::shortcut::get_all_shortcuts_for_guild(pool, guild_id.into()).await?;

        data.shortcut_cache.insert(guild_id.into(), rows);
        data.shortcut_cache
            .get(guild_id.into())
            .expect("missing cache after insertion")
    };

    // NOTE: This is the modified implementation to match KAGI's behavior.
    // It triggers the shortcut response when the message content exactly matches the trigger.
    let content = message.content.clone();
    for shortcut in shortcuts.iter() {
        if content == shortcut.trigger {
            let responses: Vec<String> = shortcut.responses();
            let response = responses
                .get(rand::rng().random_range(0..responses.len()))
                .unwrap()
                .clone();

            message.reply(ctx.http(), response).await?;
        }
    }

    // NOTE: This is the original implementation but it seems that KAGI has a different approach to it.
    // let (main_prefix, additional_prefixes) = data.config.get_prefixes();
    // let content = message.content.to_lowercase();

    // for prefix in ["/", main_prefix.as_str()]
    //     .iter()
    //     .chain(additional_prefixes.iter().map(|s| match s {
    //         poise::Prefix::Literal(s) => s,
    //         _ => panic!("Expecting Literal prefixes, received Regex"),
    //     }))
    // {
    //     if content.starts_with(prefix)
    //         && let Some(content_without_prefix) = content.strip_prefix(prefix)
    //     {
    //         for shortcut in shortcuts.iter() {
    //             if content_without_prefix.starts_with(&shortcut.trigger) {
    //                 let responses: Vec<String> = shortcut.responses();
    //                 let response = responses
    //                     .get(rand::rng().random_range(0..responses.len()))
    //                     .unwrap()
    //                     .clone();

    //                 message.reply(ctx.http(), response).await?;
    //             }
    //         }
    //     }
    // }

    Ok(())
}
