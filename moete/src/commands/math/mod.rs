mod calculation;
mod factorial;

/// Collect all commands into a single Vec
pub fn commands() -> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![calculation::calc(), factorial::factorial()]
}
