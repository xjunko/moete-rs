use crate::Error;

mod banner;
mod data;

pub use banner::banner_rotate;

pub fn commands() -> Vec<poise::Command<moete_core::State, Error>> {
    vec![]
}
