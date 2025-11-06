pub mod listener;

pub use refresh::refresh;

mod main;
mod refresh;
mod search;
mod text;

pub fn commands() -> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    // this is a bit of a oxymoronic setup but what we have here is basically
    // adding the emote-prefixed commands then
    // the subcommands without the prefix, because thats how it was
    // originally set up
    vec![main::emote(), text::text(), search::search()]
}
