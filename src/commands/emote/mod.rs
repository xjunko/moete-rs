use crate::Error;
use crate::core::State;

pub mod listener;

mod emote;
mod search;
mod text;

pub fn commands() -> Vec<poise::Command<State, Error>> {
    vec![text::text(), search::search(), emote::emote()]
}
