use sqlx::postgres;
use tracing::info;

pub async fn build(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    let res_users = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id BIGSERIAL PRIMARY KEY NOT NULL,
            count INT8 NOT NULL DEFAULT 0
        );
        "#,
    )
    .execute(pool)
    .await?;

    let res_messages = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id BIGSERIAL PRIMARY KEY NOT NULL,
            user_id INT8 REFERENCES users(id) NOT NULL,
            content TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    match (res_users.rows_affected(), res_messages.rows_affected()) {
        (0, 0) => info!("Tables 'users' and 'messages' already exist."),
        (0, _) => {
            info!("Table 'users' already exists. Created table 'messages'.")
        },
        (_, 0) => {
            info!("Created table 'users'. Table 'messages' already exists.")
        },
        _ => info!("Created tables 'users' and 'messages'."),
    }

    Ok(())
}
