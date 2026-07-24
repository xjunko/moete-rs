use sqlx::postgres;

mod configuration;
mod counter;
mod markov;
mod shortcut;

pub async fn build(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    configuration::build(pool).await?;
    counter::build(pool).await?;
    markov::build(pool).await?;
    shortcut::build(pool).await?;
    Ok(())
}
