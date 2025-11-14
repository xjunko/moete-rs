pub mod listener;

pub use refresh::refresh;

mod regexes;

mod add;
mod clone;
mod info;
mod main;
mod refresh;
mod remove;
mod search;
mod text;

pub fn commands() -> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    // this is a bit of a oxymoronic setup but what we have here is basically
    // adding the emote-prefixed commands then
    // the subcommands without the prefix, because thats how it was
    // originally set up
    vec![
        main::emote(),
        text::text(),
        text::text_as(),
        search::search(),
    ]
}
