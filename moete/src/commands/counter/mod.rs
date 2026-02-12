mod list;
pub mod listener;

pub fn commands() -> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![list::count()]
}
