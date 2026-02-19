use sqlx::postgres;

pub mod counter;
pub mod markov;
pub mod shortcut;

/// builds all the required sql tables.
pub async fn build(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    counter::build(pool).await?;
    shortcut::build(pool).await?;
    markov::build(pool).await?;
    Ok(())
}
