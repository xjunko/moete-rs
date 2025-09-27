pub mod listener;

mod main;
mod search;
mod text;

pub fn commands() -> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![main::emote()]
}
