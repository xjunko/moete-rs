use poise::serenity_prelude as serenity;
use sqlx::postgres;

pub use self::{
    branding::*,
    config::Config,
    models::*,
    state::State,
    types::*,
    {emotes::EmoteManager, models::shortcut::ShortcutCache},
};

pub mod memory;

mod branding;
mod config;
mod emotes;
mod models;
mod state;
mod types;

pub fn create_required_folders() -> std::io::Result<()> {
    std::fs::create_dir_all(".tmp/charts/")?;
    Ok(())
}

pub async fn build_sql(pool: &postgres::PgPool) -> Result<(), sqlx::Error> {
    models::markov::build(pool).await?;
    models::counter::build(pool).await?;
    models::shortcut::build(pool).await?;
    Ok(())
}
