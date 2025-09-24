pub const NAME: &str = "Moete";
pub const BRANCH: &str = "rust";
pub const VERSION: &str = "0.0.1";

pub fn name() -> String {
    NAME.to_string()
}

pub fn version() -> String {
    format!("Running on {} [{}-{}]", name(), BRANCH, VERSION)
}
