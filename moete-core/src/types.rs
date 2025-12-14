// declares all the common type used in Moete

pub type MoeteError = Box<dyn std::error::Error + Send + Sync>;
pub type MoeteContext<'a> = poise::Context<'a, super::State, MoeteError>;
