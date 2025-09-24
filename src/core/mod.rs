use sqlx::postgres;

pub use self::models::*;
pub use self::{config::Config, emotes::EmoteManager, state::State};

mod config;
mod emotes;
mod models;
mod state;

pub async fn build_sql(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    models::markov::build(pool).await?;
    models::counter::build(pool).await?;
    Ok(())
}
