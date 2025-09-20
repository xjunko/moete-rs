use sqlx::postgres;

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct User {
    pub id: i64,
    pub count: i64,
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct Message {
    pub id: i64,
    pub user_id: i64,
    pub content: String,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct MarkovUser {
    pub id: i64,
    pub count: i64,
    pub messages: Vec<Message>,
}

pub async fn get_user(
    pool: &postgres::PgPool,
    user_id: i64,
) -> Result<Option<MarkovUser>, sqlx::Error> {
    let user: User = sqlx::query_as("SELECT id, count FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    let messages: Vec<Message> = sqlx::query_as::<_, Message>(
        "SELECT id, user_id, content FROM messages WHERE user_id = $1",
    )
    .bind(user.id)
    .fetch_all(pool)
    .await?;

    Ok(Some(MarkovUser {
        id: user.id,
        count: user.count,
        messages,
    }))
}

pub async fn get_user_count(
    pool: &postgres::PgPool,
    user_id: i64,
) -> Result<Option<i64>, sqlx::Error> {
    let user: User = sqlx::query_as("SELECT id, count FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(pool)
        .await?;

    Ok(Some(user.count))
}

pub async fn add_message(
    pool: &postgres::PgPool,
    user_id: i64,
    content: &str,
) -> Result<(), sqlx::Error> {
    // tries to get user from database, if not found create new user
    let user: User = {
        match sqlx::query_as::<_, User>("SELECT id, count FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?
        {
            Some(u) => u,
            None => {
                // create new user
                sqlx::query_as("INSERT INTO users (id, count) VALUES ($1, 0)")
                    .bind(user_id)
                    .fetch_one(pool)
                    .await?
            }
        }
    };

    // add message to database
    sqlx::query("INSERT INTO messages (user_id, content) VALUES ($1, $2)")
        .bind(user.id)
        .bind(content)
        .execute(pool)
        .await?;

    // increment user's message count
    sqlx::query("UPDATE users SET count = count + 1 WHERE id = $1")
        .bind(user.id)
        .execute(pool)
        .await?;

    Ok(())
}
