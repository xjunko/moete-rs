use crate::Error;

mod google;
mod help;
mod nhentai;
mod ping;
mod urban;

/// Collect all commands into a single Vec
pub fn commands() -> Vec<poise::Command<moete_core::State, Error>> {
    vec![
        help::help(),
        google::google(),
        nhentai::nhentai(),
        ping::ping(),
        urban::urban(),
    ]
}
