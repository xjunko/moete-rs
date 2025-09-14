pub mod utility;

pub fn commands() -> Vec<poise::Command<crate::core::Data, crate::Error>> {
    let mut cmds = Vec::new();
    cmds.extend(utility::commands());
    cmds
}
