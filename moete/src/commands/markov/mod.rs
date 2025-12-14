pub mod listener;

mod learn;
mod random;
mod text;

pub fn commands() -> Vec<poise::Command<moete_core::State, moete_core::MoeteError>> {
    vec![text::markov()]
}

const ALLOWED: &[u64] = &[
    224785877086240768, // Me
    255365914898333707, // Asperl
    393201541630394368, // Nity
    261815692150571009, // Neko
    383468859211907083, // Mattsu
    546976983012212746, // Arissa
    658976239100362753, // aXccel
    223367426526412800, // Bright
    655023950689992706, // Nia
    406463187052134411, // Mooth
    315116686850129922, // Mystia
    443022415451127808, // Salty
    360256398933753856, // Zavents
    286438559341215746, // Kagi
    210065177574375424, // Eima
    369858698941693953, // Amano
    824537345541799976, // Piqi
    304512418976104450, // WilsonYogi
    922423503909687317, // Zed
    736223131240497183, // rmhakurei
];

const COMMON_BOT_PREFIXES: &[&str] = &[
    ";", "t!", "pls ", "please ", "p ", "->", "!", "`", "``", ";;", "~>", ">", "<", "$", "k!",
    ".calc", ".ss", ".google", ".",
];
