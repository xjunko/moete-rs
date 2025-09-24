use crate::Error;
use crate::core;

pub mod counter;
pub mod emote;
pub mod markov;
pub mod pakb;
pub mod role;
pub mod utility;

pub fn commands() -> Vec<poise::Command<core::State, Error>> {
    let mut cmds = Vec::new();
    cmds.extend(utility::commands());
    cmds.extend(emote::commands());
    cmds.extend(role::commands());
    cmds.extend(markov::commands());
    cmds.extend(pakb::commands());
    cmds.extend(counter::commands());
    cmds
}
