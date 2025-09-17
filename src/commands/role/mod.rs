use crate::{Error, core::State};

mod color;
mod list;

pub fn commands() -> Vec<poise::Command<State, Error>> {
    vec![color::color(), list::list()]
}
