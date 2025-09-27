use crate::Error;

mod color;
mod list;

pub fn commands() -> Vec<poise::Command<moete_core::State, Error>> {
    vec![color::color()]
}
