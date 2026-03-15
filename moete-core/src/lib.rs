use poise::serenity_prelude as serenity;

pub use self::branding::*;
pub use self::config::Config;
pub use self::emotes::EmoteManager;
pub use self::state::State;
pub use self::types::*;

pub mod memory;

mod branding;
mod config;
mod emotes;
mod state;
mod types;

pub fn create_required_folders() -> std::io::Result<()> {
    std::fs::create_dir_all(".tmp/charts/")?;
    Ok(())
}
