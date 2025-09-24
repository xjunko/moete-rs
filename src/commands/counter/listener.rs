use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;

use crate::core;
use crate::{Error, serenity};

pub static WORDS: Lazy<HashMap<String, Vec<String>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    let words_env = env::var("INSTANCE_WORD_LISTS").unwrap_or("".to_string());

    for entry in words_env.split("|") {
        let (main, variations) = entry.split_once(":").unwrap();
        map.insert(
            main.to_string(),
            variations.split(",").map(|s| s.to_string()).collect(),
        );
    }
    map
});

pub static FLATTEN_WORDS: Lazy<Vec<String>> = Lazy::new(|| {
    let mut vec = Vec::new();

    for (main, variations) in &*WORDS {
        vec.push(main.clone());
        vec.extend(variations.clone());
    }

    vec
});

pub async fn on_message(
    _ctx: &serenity::Context,
    message: &serenity::Message,
    data: &core::State,
) -> Result<(), Error> {
    if FLATTEN_WORDS
        .iter()
        .any(|w| message.content.to_lowercase().contains(w))
    {
        // ok great, now we just have to find the main word.
        let mut to_increment: Vec<String> = Vec::new();

        for (main, variations) in &*WORDS {
            for variation in variations {
                if message.content.to_lowercase().contains(variation) {
                    to_increment.push(main.clone());
                    break;
                }
            }
        }

        // increment in database
        if let Some(pool) = data.pool.as_ref() {
            for word in to_increment {
                core::counter::increment_word_for_user_id(&pool, message.author.id.into(), &word)
                    .await?;
            }
        }
    }
    Ok(())
}
