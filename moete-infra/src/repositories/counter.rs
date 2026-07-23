use sqlx::postgres;

use crate::counter::Counter;

pub async fn find(
    pool: &postgres::PgPool,
    user_id: i64,
    word: &str,
) -> Result<Option<Counter>, sqlx::Error> {
    sqlx::query_as(
        "SELECT user_id, word, count
        FROM counters
        WHERE user_id = $1 AND word = $2",
    )
    .bind(user_id)
    .bind(word)
    .fetch_optional(pool)
    .await
}

pub async fn create(
    pool: &postgres::PgPool,
    user_id: i64,
    word: &str,
) -> Result<Counter, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO counters (user_id, word, count)
        VALUES ($1, $2, 0)
        RETURNING user_id, word, count",
    )
    .bind(user_id)
    .bind(word)
    .fetch_one(pool)
    .await
}

pub async fn increment(
    pool: &postgres::PgPool,
    user_id: i64,
    word: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE counters
        SET count = count + 1
        WHERE user_id = $1 AND word = $2",
    )
    .bind(user_id)
    .bind(word)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn find_by_word(
    pool: &postgres::PgPool,
    word: &str,
) -> Result<Vec<Counter>, sqlx::Error> {
    sqlx::query_as(
        "SELECT user_id, word, count
        FROM counters
        WHERE word = $1
        ORDER BY count DESC",
    )
    .bind(word)
    .fetch_all(pool)
    .await
}
