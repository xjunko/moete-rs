use crate::Error;

mod list;
pub mod listener;

pub fn commands() -> Vec<poise::Command<moete_core::State, Error>> {
    vec![list::count()]
}
