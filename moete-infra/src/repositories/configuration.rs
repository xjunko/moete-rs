use sqlx::postgres;
use tracing::info;

use crate::configuration::{
    Configuration,
    Server,
};

pub async fn find(
    pool: &postgres::PgPool,
    guild_id: i64,
) -> Result<Option<Configuration>, sqlx::Error> {
    let config: Option<Configuration> = sqlx::query_as(
        "SELECT guild_id, version, server FROM configuration WHERE guild_id = $1",
    )
    .bind(guild_id)
    .fetch_optional(pool)
    .await?;

    let Some(mut cfg) = config else {
        return Ok(None);
    };

    if cfg.version < Server::VERSION {
        info!(
            "Backfilling configuration for guild_id {} ({} -> {})",
            guild_id,
            cfg.version,
            Server::VERSION
        );
        cfg.version = Server::VERSION;
        cfg = update(pool, &cfg).await?;
    }

    Ok(Some(cfg))
}

pub async fn create(
    pool: &postgres::PgPool,
    config: &Configuration,
) -> Result<Configuration, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO configuration (guild_id, version, server)
        VALUES ($1, $2, $3)
        ON CONFLICT (guild_id) DO UPDATE SET guild_id = EXCLUDED.guild_id
        RETURNING guild_id, version, server",
    )
    .bind(config.guild_id)
    .bind(config.version)
    .bind(&config.server)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &postgres::PgPool,
    config: &Configuration,
) -> Result<Configuration, sqlx::Error> {
    sqlx::query_as(
        "UPDATE configuration
        SET version = $3, server = $2
        WHERE guild_id = $1
        RETURNING guild_id, version, server",
    )
    .bind(config.guild_id)
    .bind(&config.server)
    .bind(config.version)
    .fetch_one(pool)
    .await
}
