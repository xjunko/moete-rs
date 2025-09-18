use crate::Error;
use crate::core::State;

mod text;

pub fn commands() -> Vec<poise::Command<State, Error>> {
    vec![text::markov()]
}
