pub mod emote;
pub mod markov;
pub mod role;
pub mod utility;

pub fn commands() -> Vec<poise::Command<crate::core::State, crate::Error>> {
    let mut cmds = Vec::new();
    cmds.extend(utility::commands());
    cmds.extend(emote::commands());
    cmds.extend(role::commands());
    cmds.extend(markov::commands());
    cmds
}
