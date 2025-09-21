use crate::Error;
use crate::core;

mod banner;
mod data;

pub use banner::banner_rotate;

pub fn commands() -> Vec<poise::Command<core::State, Error>> {
    vec![]
}
