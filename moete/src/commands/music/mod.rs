mod events;
mod join;
mod leave;
mod play;
mod stop;

pub fn commands()
-> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![play::play(), join::join(), leave::leave(), stop::stop()]
}
