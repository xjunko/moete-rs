use crate::{Error, core::State};

mod banner;
mod data;

pub use banner::banner_rotate;

pub fn commands() -> Vec<poise::Command<State, Error>> {
    vec![]
}
