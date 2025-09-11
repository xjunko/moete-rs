use crate::{Error, State};

mod google;
pub use google::*;

mod help;
pub use help::*;

mod ping;
pub use ping::*;

mod urban;
pub use urban::*;

/// Collect all commands into a single Vec
pub fn commands() -> Vec<poise::Command<State, Error>> {
    vec![help(), google(), ping(), urban()]
}
