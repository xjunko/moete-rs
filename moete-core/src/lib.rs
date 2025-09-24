use poise::serenity_prelude as serenity;
use sqlx::postgres;

pub use self::branding::*;
pub use self::models::*;
pub use self::{config::Config, emotes::EmoteManager, state::State};

mod branding;
mod config;
mod emotes;
mod models;
mod state;

type Error = moete_framework::MoeteError;

pub async fn build_sql(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    models::markov::build(pool).await?;
    models::counter::build(pool).await?;
    Ok(())
}
