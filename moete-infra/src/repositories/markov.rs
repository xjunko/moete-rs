use sqlx::postgres;

use crate::markov::{
    Message,
    User,
};

pub async fn create_user(
    pool: &postgres::PgPool,
    user_id: i64,
) -> Result<User, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO users (id, count)
        VALUES ($1, 0)
        RETURNING id, count",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn find_user(
    pool: &postgres::PgPool,
    user_id: i64,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, count
    FROM users
    WHERE id = $1",
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_message(
    pool: &postgres::PgPool,
    user_id: i64,
    content: &str,
) -> Result<Message, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO messages (user_id, content)
        VALUES ($1, $2)
        RETURNING id, user_id, content",
    )
    .bind(user_id)
    .bind(content)
    .fetch_one(pool)
    .await
}

pub async fn find_messages(
    pool: &postgres::PgPool,
    user_id: i64,
) -> Result<Vec<Message>, sqlx::Error> {
    sqlx::query_as(
        "SELECT id, user_id, content
    FROM messages
    WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn find_message_count(
    pool: &postgres::PgPool,
    user_id: i64,
) -> Result<i64, sqlx::Error> {
    let count: (i64,) = sqlx::query_as(
        "SELECT count(*) 
        FROM messages 
        WHERE user_id = $1",
    )
    .bind(user_id)
    .fetch_one(pool)
    .await?;

    Ok(count.0)
}

pub async fn increment_message_count(
    pool: &postgres::PgPool,
    user_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users
        SET count = count + 1
        WHERE id = $1",
    )
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}
