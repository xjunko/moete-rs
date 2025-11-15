use poise::serenity_prelude as serenity;
use sqlx::postgres;

pub use self::{
    branding::*, config::Config, emotes::EmoteManager, models::*, state::State, types::*,
};

mod branding;
mod config;
mod emotes;
mod models;
mod state;
mod types;

pub async fn build_sql(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    models::markov::build(pool).await?;
    models::counter::build(pool).await?;
    Ok(())
}
