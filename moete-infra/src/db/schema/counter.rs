use sqlx::postgres;
use tracing::info;

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
