use sqlx::postgres;

use crate::shortcut::Shortcut;

pub async fn find(
    pool: &postgres::PgPool,
    guild_id: i64,
    trigger: &str,
) -> Result<Option<Shortcut>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, guild_id, trigger, response 
        FROM shortcuts
        WHERE guild_id = $1 AND trigger = $2",
    )
    .bind(guild_id)
    .bind(trigger)
    .fetch_optional(pool)
    .await
}

pub async fn find_all(
    pool: &postgres::PgPool,
    guild_id: i64,
) -> Result<Vec<Shortcut>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, guild_id, trigger, response 
        FROM shortcuts
        WHERE guild_id = $1",
    )
    .bind(guild_id)
    .fetch_all(pool)
    .await
}

pub async fn create(
    pool: &postgres::PgPool,
    guild_id: i64,
    trigger: &str,
    response: &str,
) -> Result<Shortcut, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO shortcuts (guild_id, trigger, response)
        VALUES ($1, $2, $3)
        RETURNING id, guild_id, trigger, response",
    )
    .bind(guild_id)
    .bind(trigger)
    .bind(response)
    .fetch_one(pool)
    .await
}

pub async fn update(
    pool: &postgres::PgPool,
    guild_id: i64,
    trigger: &str,
    response: &str,
) -> Result<Shortcut, sqlx::Error> {
    sqlx::query_as(
        "UPDATE shortcuts
        SET response = $3
        WHERE guild_id = $1 AND trigger = $2
        RETURNING id, guild_id, trigger, response",
    )
    .bind(guild_id)
    .bind(trigger)
    .bind(response)
    .fetch_one(pool)
    .await
}

pub async fn remove(
    pool: &postgres::PgPool,
    guild_id: i64,
    trigger: &str,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query(
        "DELETE FROM shortcuts
        WHERE guild_id = $1 AND trigger = $2",
    )
    .bind(guild_id)
    .bind(trigger)
    .execute(pool)
    .await?;

    Ok(res.rows_affected())
}

pub async fn find_all_guilds_id(
    pool: &postgres::PgPool,
) -> Result<Vec<i64>, sqlx::Error> {
    sqlx::query_as::<_, (i64,)>("SELECT DISTINCT guild_id FROM shortcuts")
        .fetch_all(pool)
        .await
        .map(|rows| rows.into_iter().map(|(guild_id,)| guild_id).collect())
}
