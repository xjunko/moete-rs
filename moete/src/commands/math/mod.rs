use crate::Error;

mod calculation;
mod factorial;

/// Collect all commands into a single Vec
pub fn commands() -> Vec<poise::Command<moete_core::State, Error>> {
    vec![calculation::calc(), factorial::factorial()]
}
