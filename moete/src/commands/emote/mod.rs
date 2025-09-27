use crate::Error;

pub mod listener;

mod main;
mod search;
mod text;

pub fn commands() -> Vec<poise::Command<moete_core::State, Error>> {
    vec![main::emote()]
}
