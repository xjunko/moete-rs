pub mod listener;
mod macros;

pub fn commands() -> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![macros::shortcut()]
}
