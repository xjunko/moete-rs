use poise::serenity_prelude as serenity;

type Error = moete_framework::MoeteError;
type Context<'a> = moete_framework::MoeteContext<'a, moete_core::State, Error>;

pub mod cdn;
pub mod color;
pub mod embed;
pub mod help;
pub mod paginate;
pub mod poise_builtins;
pub mod user;
pub mod webhook;
