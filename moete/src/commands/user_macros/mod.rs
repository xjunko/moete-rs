pub mod listener;
mod macros;

#[cfg(feature = "macros")]
pub mod commands;

pub const MAX_LENGTH: usize = 1024;

pub fn commands()
-> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![macros::shortcut()]
}
