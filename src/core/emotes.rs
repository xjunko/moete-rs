use crate::Error;
use crate::serenity;
use tracing::info;

/// EmoteManager handles all emoji related operation.
pub struct EmoteManager {
    /// Came straight from the bot's application
    internal: Vec<serenity::Emoji>,
    /// Whitelisted servers
    external: Vec<serenity::Emoji>,
}

impl EmoteManager {
    pub fn new() -> Self {
        EmoteManager {
            internal: Vec::new(),
            external: Vec::new(),
        }
    }

    /// Fetches the bot emojis from Discord.
    async fn fetch_bot_emojis(ctx: &serenity::Context) -> Result<Vec<serenity::Emoji>, Error> {
        let http: &serenity::Http = &ctx.http;
        let emojis: Vec<serenity::Emoji> = http.get_application_emojis().await?;
        Ok(emojis)
    }

    /// Returns an iterator over all emojis the bot can access.
    pub fn global(&self) -> impl Iterator<Item = &serenity::Emoji> {
        self.internal.iter().chain(self.external.iter())
    }

    /// Load all the emojis we can use into EmoteManager.
    pub async fn load(&mut self, ctx: &serenity::Context) {
        self.internal = Self::fetch_bot_emojis(ctx).await.unwrap_or_default();
        info!("Loaded {} bot emojis", self.internal.len());
    }

    /// Returns emoji if the word matches an emoji name.
    pub fn get(&self, name: &str) -> Option<&serenity::Emoji> {
        self.global()
            .filter(|e| e.available)
            .find(|e| e.name.eq_ignore_ascii_case(name))
    }

    /// Returns emojis matching the names.
    pub fn get_many(&self, query: &str) -> Vec<&serenity::Emoji> {
        self.global()
            .filter(|e| e.available)
            .filter(|e| e.name.contains(query))
            .collect()
    }

    /// Builds a text and transform it with emojis we can use.
    pub fn text(&self, text: &str) -> String {
        let content: Vec<String> = text
            .split(" ")
            .map(|word| {
                if let Some(e) = self.get(word) {
                    e.to_string()
                } else {
                    word.to_string()
                }
            })
            .collect();
        content.join(" ")
    }
}
