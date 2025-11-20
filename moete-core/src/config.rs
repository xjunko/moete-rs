use std::env;

#[allow(dead_code)]
#[derive(Debug)]

pub struct Flag {
    pub debug: bool,
    pub minimal: bool,
}

impl Default for Flag {
    fn default() -> Self {
        Self {
            debug: env::var("IS_DEBUG")
                .unwrap_or("False".to_string())
                .to_lowercase()
                .eq("true"),
            minimal: env::var("IS_MINIMAL")
                .unwrap_or("False".to_string())
                .to_lowercase()
                .eq("true"),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]

pub struct Moete {
    pub owners: &'static [u64],
    pub blacklisted: &'static [u64],
    pub whitelisted: &'static [&'static str],
    pub owned: &'static [&'static str],
    pub status: &'static [&'static str],
}

impl Default for Moete {
    fn default() -> Self {
        Self {
            owners: &[
                224785877086240768, // xjunko
                255365914898333707,
                393201541630394368,
                261815692150571009,
                302755468802260993,
                872808388562661446,
                223367426526412800,
                360256398933753856,
                406463187052134411,
                315116686850129922,
                546976983012212746,
                443022415451127808,
                736223131240497183, // rmhakurei
            ],
            blacklisted: &[
                924728250666733578, // aya server, she asked for this
            ],
            whitelisted: &[
                "CustomEmote",
                "PAKB",
                "EmoteBot",
                "HololiveEmote",
                "709407328406994975",
                "696405704134885466",
                "747623770876805193", // Zavents Emote Server
            ],
            owned: &["EmoteServer"],
            status: &[
                "W|the moon",
                "W|the stars",
                "W|you",
                "W|the world",
                "W|for any new messages",
                "L|FLAVOR FOLEY - weathergirl",
                "P|with your feelings",
                "P|Minecraft",
                "P|osu!",
                "L|the rain",
                "L|the voices in my head",
            ],
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Discord {
    pub name: String,
    pub token: String,
    pub token_cdn: String,
    pub prefixes: Vec<String>,
    pub owner: u64,
}

impl Default for Discord {
    fn default() -> Self {
        Self {
            name: env::var("INSTANCE_NAME").unwrap_or("Moete".to_string()),
            token: env::var("INSTANCE_TOKEN_DISCORD")
                .expect("Error: INSTANCE_TOKEN_DISCORD not set"),
            token_cdn: env::var("INSTANCE_TOKEN_CDN").expect("Error: INSTANCE_TOKEN_CDN not set"),
            prefixes: env::var("INSTANCE_PREFIXES")
                .unwrap_or(";".to_string())
                .split(" ")
                .map(|s| s.to_string())
                .collect(),
            owner: env::var("INSTANCE_OWNER_DISCORD")
                .expect("Error: INSTANCE_OWNER_DISCORD not set")
                .parse()
                .expect("Error: INSTANCE_OWNER_DISCORD must be a valid u64"),
        }
    }
}

#[derive(Debug)]
pub struct Services {
    pub database: String,
}

impl Default for Services {
    fn default() -> Self {
        Self {
            database: env::var("INSTANCE_DB_URL")
                .expect("INSTANCE_DB_URL must be set in the environment"),
        }
    }
}

#[derive(Default, Debug)]
pub struct Config {
    pub flag: Flag,
    pub moete: Moete,
    pub discord: Discord,
    pub services: Services,
}

impl Config {
    pub fn get_prefixes(&self) -> (String, Vec<poise::Prefix>) {
        if self.flag.debug {
            return (";;".to_string(), vec![poise::Prefix::Literal("moete@")]);
        }
        // FIXME: make this use self.discord.prefixes
        (
            ";".to_string(),
            vec![
                poise::Prefix::Literal(":"),
                poise::Prefix::Literal("#"),
                poise::Prefix::Literal("e!"),
                poise::Prefix::Literal("e#"),
            ],
        )
    }

    pub fn get_status(&self) -> &'static [&'static str] {
        if self.flag.debug {
            return &["W|Debug Mode"];
        }

        self.moete.status
    }
}
