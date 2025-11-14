pub const NAME: &str = "Moete";
pub const BRANCH: &str = "rust";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const GIT: &str = env!("GIT_HASH");

pub fn name() -> String {
    NAME.to_string()
}

pub fn version() -> String {
    format!(
        "Running on {} [{}-{}] (commit: {})",
        name(),
        BRANCH,
        VERSION,
        GIT
    )
}
