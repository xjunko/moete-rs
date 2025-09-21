use crate::Error;
use crate::core;

pub mod listener;

mod main;
mod search;
mod text;

pub fn commands() -> Vec<poise::Command<core::State, Error>> {
    vec![text::text(), search::search(), main::emote()]
}
