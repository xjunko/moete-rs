use crate::Error;
use crate::core;

mod list;
pub mod listener;

pub fn commands() -> Vec<poise::Command<core::State, Error>> {
    vec![list::count()]
}
