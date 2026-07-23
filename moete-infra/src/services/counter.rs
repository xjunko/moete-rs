use sqlx::postgres;

use crate::repositories;

pub async fn increment_word_for_user_id(
    pool: &postgres::PgPool,
    user_id: i64,
    word: &str,
) -> Result<(), sqlx::Error> {
    let exists = repositories::counter::find(pool, user_id, word).await?;

    if exists.is_none() {
        repositories::counter::create(pool, user_id, word).await?;
    }

    repositories::counter::increment(pool, user_id, word).await?;

    Ok(())
}

pub async fn get_counters(
    pool: &postgres::PgPool,
    word: &str,
) -> Result<Vec<crate::counter::Counter>, sqlx::Error> {
    repositories::counter::find_by_word(pool, word).await
}
