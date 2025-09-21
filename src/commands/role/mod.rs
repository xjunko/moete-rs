use crate::Error;
use crate::core;

mod color;
mod list;

pub fn commands() -> Vec<poise::Command<core::State, Error>> {
    vec![color::color(), list::list()]
}
