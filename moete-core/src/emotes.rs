use std::sync::Arc;
use tracing::{debug, error};

use super::Config;
use super::MoeteError;
use super::serenity;

/// EmoteManager handles all emoji related operation.
#[derive(Default, Debug)]
pub struct EmoteManager {
    /// Came straight from the bot's application
    internal: Vec<serenity::Emoji>,
    /// Whitelisted servers
    external: Vec<serenity::Emoji>,
}

impl EmoteManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Fetches the bot emojis from Discord.
    async fn fetch_bot_emojis(ctx: &serenity::Context) -> Result<Vec<serenity::Emoji>, MoeteError> {
        let http: &serenity::Http = &ctx.http;
        let emojis: Vec<serenity::Emoji> = http.get_application_emojis().await?;
        Ok(emojis)
    }

    /// Fetches emojis from a guild by its ID.
    async fn fetch_guild_emojis_by_id(
        ctx: &serenity::Context,
        id: u64,
    ) -> Result<Vec<serenity::Emoji>, MoeteError> {
        let http: &serenity::Http = &ctx.http;
        let emojis: Vec<serenity::Emoji> = http.get_emojis(serenity::GuildId::new(id)).await?;
        Ok(emojis)
    }

    /// Fetches emojis from a guild by its name.
    async fn fetch_guild_emojis_by_name(
        ctx: &serenity::Context,
        query: &str,
    ) -> Result<Vec<serenity::Emoji>, MoeteError> {
        let http: &serenity::Http = &ctx.http;
        let cache: &serenity::Cache = &ctx.cache;
        let mut emojis: Vec<serenity::Emoji> = Vec::new();

        for guild_id in cache.guilds() {
            if let Some(name) = guild_id.name(cache) {
                if name.contains(query) {
                    let found: Vec<serenity::Emoji> = http
                        .get_emojis(serenity::GuildId::new(guild_id.get()))
                        .await?;
                    emojis.extend(found);
                }
            } else {
                error!("Guild name not found in cache: {:?}", guild_id);
            }
        }
        Ok(emojis)
    }

    /// Fetches all the emojis that the bot can access.
    async fn fetch_guild_emojis(
        ctx: &serenity::Context,
        config: Arc<Config>,
    ) -> Result<Vec<serenity::Emoji>, MoeteError> {
        let mut emojis: Vec<serenity::Emoji> = Vec::new();

        for query in config
            .moete
            .whitelisted
            .iter()
            .chain(config.moete.owned.iter())
        {
            if let Ok(id) = query.parse::<u64>() {
                let found = Self::fetch_guild_emojis_by_id(ctx, id).await?;
                debug!("Found {} emojis in guild ID={}", found.len(), id);
                emojis.extend(found);
            } else {
                let found = Self::fetch_guild_emojis_by_name(ctx, query).await?;
                debug!("Found {} emojis in guild Query={}", found.len(), query);
                emojis.extend(found);
            }
        }

        Ok(emojis)
    }

    /// Returns an iterator over all emojis the bot can access.
    pub fn global(&self) -> impl Iterator<Item = &serenity::Emoji> {
        // NOTE: tbh i would add a sort here but
        //       it needs allocation so...
        self.internal.iter().chain(self.external.iter())
    }

    /// The inner load function.
    async fn load_inner(&mut self, ctx: &serenity::Context, config: Arc<Config>) {
        self.internal = Self::fetch_bot_emojis(ctx).await.unwrap_or_default();
        self.external = Self::fetch_guild_emojis(ctx, config)
            .await
            .unwrap_or_default();

        self.internal.sort_by(|a, b| a.name.cmp(&b.name));
        self.external.sort_by(|a, b| a.name.cmp(&b.name));

        debug!("Loaded {} bot emojis", self.internal.len());
        debug!("Loaded {} external emojis", self.external.len());
        debug!("Total {} emojis available", self.global().count());
    }

    /// Load the emojis we can use.
    pub async fn load(&mut self, ctx: &serenity::Context, config: Arc<Config>) {
        self.load_inner(ctx, config).await;
    }

    /// Refresh the emojis we can use.
    /// This will re-fetch all emojis from Discord.
    pub async fn refresh(&mut self, ctx: &serenity::Context, config: Arc<Config>) {
        self.load_inner(ctx, config).await;
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
