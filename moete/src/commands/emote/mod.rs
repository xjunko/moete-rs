pub mod listener;

pub use refresh::refresh;

mod main;
mod refresh;
mod search;
mod text;

pub fn commands() -> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![main::emote()]
}
