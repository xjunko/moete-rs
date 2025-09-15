use poise::Prefix;

pub fn debug_prefixes() -> (String, Vec<Prefix>) {
    (String::from(";;"), vec![Prefix::Literal("moete@")])
}
pub fn release_prefixes() -> (String, Vec<Prefix>) {
    (
        String::from(";"),
        vec![
            Prefix::Literal(":"),
            Prefix::Literal("#"),
            Prefix::Literal("e!"),
            Prefix::Literal("e#"),
        ],
    )
}
