use poise::Prefix;

pub fn debug_prefixes() -> (String, Vec<Prefix>) {
    (String::from(";;"), vec![Prefix::Literal("moete@".into())])
}
pub fn release_prefixes() -> (String, Vec<Prefix>) {
    (
        String::from(";"),
        vec![
            Prefix::Literal(":".into()),
            Prefix::Literal("#".into()),
            Prefix::Literal("e!".into()),
            Prefix::Literal("e#".into()),
        ],
    )
}
