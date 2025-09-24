use crate::Error;
use crate::core;

mod calculation;
mod factorial;

/// Collect all commands into a single Vec
pub fn commands() -> Vec<poise::Command<core::State, Error>> {
    vec![calculation::calc(), factorial::factorial()]
}
