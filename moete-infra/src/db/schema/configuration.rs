use sqlx::postgres;
use tracing::info;

pub async fn build(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    let res_counters = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS configuration (
            guild_id BIGINT PRIMARY KEY,
            version INT8 NOT NULL DEFAULT 1,
            server JSONB NOT NULL,
            updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
        );"#,
    )
    .execute(pool)
    .await?;

    match res_counters.rows_affected() {
        0 => info!("Table 'configuration' already exists."),
        _ => info!("Created table 'configuration'."),
    }

    Ok(())
}
