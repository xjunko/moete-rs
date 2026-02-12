mod color;
mod list;
mod remove;

pub mod housekeeping;

pub fn commands() -> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![color::color()]
}
