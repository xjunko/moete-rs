use sqlx::postgres;
use tracing::info;

pub async fn build(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    let res_shortcuts = sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shortcuts (
            id BIGSERIAL PRIMARY KEY NOT NULL,
            guild_id INT8 NOT NULL,
            trigger TEXT NOT NULL,
            response TEXT NOT NULL,
            CONSTRAINT unique_trigger_per_guild UNIQUE (guild_id, trigger)
        );
        "#,
    )
    .execute(pool)
    .await?;

    match res_shortcuts.rows_affected() {
        0 => info!("Table 'shortcuts' already exists."),
        _ => info!("Created table 'shortcuts'."),
    }

    Ok(())
}
