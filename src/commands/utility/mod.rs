use crate::{Error, core::State};

mod google;
mod help;
mod nhentai;
mod ping;
mod urban;

/// Collect all commands into a single Vec
pub fn commands() -> Vec<poise::Command<State, Error>> {
    vec![
        help::help(),
        google::google(),
        nhentai::nhentai(),
        ping::ping(),
        urban::urban(),
    ]
}
