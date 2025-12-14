use sqlx::postgres;
use tracing::info;

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct Counter {
    pub user_id: i64,
    pub word: String,
    pub count: i64,
}

pub async fn build(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    let res_counters = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS counters (
            user_id BIGSERIAL NOT NULL,
            word TEXT NOT NULL,
            count INT8 NOT NULL DEFAULT 0,
            CONSTRAINT unique_user UNIQUE (user_id, word)
        );
        "#,
    )
    .execute(pool)
    .await?;

    match res_counters.rows_affected() {
        0 => info!("Table 'counters' already exists."),
        _ => info!("Created table 'counters'."),
    }

    Ok(())
}

pub async fn increment_word_for_user_id(
    pool: &postgres::PgPool,
    user_id: i64,
    word: &str,
) -> Result<(), sqlx::Error> {
    // check if user is in db, if not insert
    let counter: Counter = {
        match sqlx::query_as(
            "SELECT user_id, word, count FROM counters WHERE user_id = $1 AND word = $2",
        )
        .bind(user_id)
        .bind(word)
        .fetch_optional(pool)
        .await?
        {
            Some(u) => u,
            None => {
                sqlx::query_as("INSERT INTO counters (user_id, word, count) VALUES ($1, $2, 0) RETURNING user_id, word, count")
                    .bind(user_id)
                    .bind(word)
                    .fetch_one(pool)
                    .await?
            }
        }
    };

    // increment count
    sqlx::query("UPDATE counters SET count = count + 1 WHERE user_id = $1 AND word = $2")
        .bind(counter.user_id)
        .bind(counter.word)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_counters(
    pool: &postgres::PgPool,
    word: &str,
) -> Result<Vec<Counter>, sqlx::Error> {
    let counters: Vec<Counter> = sqlx::query_as(
        "SELECT user_id, word, count FROM counters WHERE word = $1 ORDER BY count DESC",
    )
    .bind(word)
    .fetch_all(pool)
    .await?;
    Ok(counters)
}
