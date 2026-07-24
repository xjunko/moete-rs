mod configuration;
mod currency;
mod google;
mod help;
mod invite;
mod machine;
mod nhentai;
mod ping;
mod server;
mod urban;

/// Collect all commands into a single Vec
pub fn commands()
-> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![
        help::help(),
        configuration::configuration(),
        google::google(),
        nhentai::nhentai(),
        ping::ping(),
        urban::urban(),
        invite::invite(),
        machine::machine(),
        server::server(),
        currency::convert(),
    ]
}
