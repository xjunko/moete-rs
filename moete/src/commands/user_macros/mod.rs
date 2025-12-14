pub mod listener;
mod main;

pub fn commands() -> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![main::shortcut()]
}
