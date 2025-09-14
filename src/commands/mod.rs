pub mod emote;
pub mod utility;

pub fn commands() -> Vec<poise::Command<crate::core::State, crate::Error>> {
    let mut cmds = Vec::new();
    cmds.extend(utility::commands());
    cmds.extend(emote::commands());
    cmds
}
