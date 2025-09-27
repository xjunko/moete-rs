use crate::Error;

pub mod counter;
pub mod emote;
pub mod markov;
pub mod math;
pub mod pakb;
pub mod role;
pub mod utility;

pub fn commands() -> Vec<poise::Command<moete_core::State, Error>> {
    let mut cmds = Vec::new();
    cmds.extend(emote::commands());
    cmds.extend(math::commands());
    cmds.extend(counter::commands());
    cmds.extend(pakb::commands());
    cmds.extend(utility::commands());
    cmds.extend(markov::commands());
    cmds.extend(role::commands());

    cmds
}
