use sqlx::postgres;

use crate::markov::MarkovUser;
use crate::repositories;

pub async fn get_user(
    pool: &postgres::PgPool,
    user_id: i64,
) -> Result<Option<MarkovUser>, sqlx::Error> {
    if let Ok(user) = repositories::markov::find_user(pool, user_id).await
        && let Some(user) = user
        && let Ok(messages) =
            repositories::markov::find_messages(pool, user_id).await
    {
        Ok(Some(MarkovUser::from((user, messages))))
    } else {
        Ok(None)
    }
}

pub async fn get_user_count(
    pool: &postgres::PgPool,
    user_id: i64,
) -> Result<Option<i64>, sqlx::Error> {
    if let Ok(count) =
        repositories::markov::find_message_count(pool, user_id).await
    {
        Ok(Some(count))
    } else {
        Ok(None)
    }
}

pub async fn add_message(
    pool: &postgres::PgPool,
    user_id: i64,
    content: &str,
) -> Result<(), sqlx::Error> {
    if let Ok(user) = repositories::markov::find_user(pool, user_id).await
        && let Some(user) = user
    {
        repositories::markov::create_message(pool, user.id, content).await?;
        repositories::markov::increment_message_count(pool, user.id).await?;
        Ok(())
    } else {
        repositories::markov::create_user(pool, user_id).await?;
        repositories::markov::create_message(pool, user_id, content).await?;
        repositories::markov::increment_message_count(pool, user_id).await?;
        Ok(())
    }
}
