use std::sync::OnceLock;

use moka::future::Cache;
use sqlx::postgres;

use crate::configuration::{
    Configuration,
    Server,
};
use crate::repositories;

static CONFIG_CACHE: OnceLock<Cache<i64, Configuration>> = OnceLock::new();

fn cache() -> &'static Cache<i64, Configuration> {
    CONFIG_CACHE.get_or_init(|| {
        Cache::builder()
            .max_capacity(32)
            .time_to_idle(std::time::Duration::from_secs(60))
            .build()
    })
}

pub async fn get(
    pool: &postgres::PgPool,
    guild_id: i64,
) -> Result<Configuration, sqlx::Error> {
    if let Some(config) = cache().get(&guild_id).await {
        return Ok(config);
    }

    let config = match repositories::configuration::find(pool, guild_id).await?
    {
        Some(config) => config,
        None => {
            let config = Configuration { guild_id, ..Default::default() };
            repositories::configuration::create(pool, &config).await?;
            config
        },
    };

    cache().insert(guild_id, config.clone()).await;
    Ok(config)
}

pub async fn update(
    pool: &postgres::PgPool,
    config: &Configuration,
) -> Result<Configuration, sqlx::Error> {
    let updated = repositories::configuration::update(pool, config).await?;
    cache().insert(config.guild_id, updated.clone()).await;
    Ok(updated)
}

pub async fn update_with<F>(
    pool: &postgres::PgPool,
    guild_id: i64,
    mutate: F,
) -> Result<Configuration, sqlx::Error>
where
    F: FnOnce(&mut Server),
{
    let mut config = get(pool, guild_id).await?;
    mutate(&mut config.server.0);
    update(pool, &config).await
}
