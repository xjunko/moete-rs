pub type MoeteError = Box<dyn std::error::Error + Send + Sync>;
pub type MoeteContext<'a, S, E> = poise::Context<'a, S, E>;
