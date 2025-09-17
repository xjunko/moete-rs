use crate::{Error, core::State};

mod google;
mod help;
mod ping;
mod urban;

/// Collect all commands into a single Vec
pub fn commands() -> Vec<poise::Command<State, Error>> {
    vec![help::help(), google::google(), ping::ping(), urban::urban()]
}
