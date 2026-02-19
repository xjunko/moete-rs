use sqlx::postgres;

pub use self::models::*;

mod models;

pub async fn build(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    models::counter::build(pool).await?;
    models::shortcut::build(pool).await?;
    models::markov::build(pool).await?;
    Ok(())
}
