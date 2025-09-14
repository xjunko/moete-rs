use crate::Error;
use crate::core::State;

pub mod listener;
mod text;

pub fn commands() -> Vec<poise::Command<State, Error>> {
    vec![text::text()]
}
