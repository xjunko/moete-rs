use serde::{
    Deserialize,
    Serialize,
};
use sqlx::types::Json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Setting<T> {
    pub value: T,
    pub description: String,
}

#[derive(Debug, thiserror::Error)]
pub enum SetFieldError {
    #[error("unknown configuration option: `{0}`")]
    UnknownOption(String),
    #[error("invalid value `{1}` for `{0}`")]
    InvalidValue(String, String),
}

macro_rules! define_server {
    (version: $version:expr; $( $field:ident : $ty:ty = $default:expr, $desc:literal );* $(;)?) => {
        #[derive(Debug, Serialize, Clone)]
        pub struct Server {
            $( pub $field: Setting<$ty>, )*
        }

        impl Server {
            pub const VERSION: i64 = $version;
        }

        impl Default for Server {
            fn default() -> Self {
                Server {
                    $( $field: Setting { value: $default, description: $desc.into() }, )*
                }
            }
        }

        impl<'de> Deserialize<'de> for Server {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::de::Error as _;

                // this is stupid as fuck but we gotta do this to backfill new fields
                let mut map = serde_json::Map::<String, serde_json::Value>::deserialize(deserializer)?;

                Ok(Server {
                    $(
                        $field: match map.remove(stringify!($field)) {
                            Some(v) => serde_json::from_value(v).map_err(|e| {
                                D::Error::custom(format!(
                                    "invalid value for `{}`: {}",
                                    stringify!($field),
                                    e
                                ))
                            })?,
                            None => Setting { value: $default, description: $desc.into() },
                        },
                    )*
                })
            }
        }

        impl Server {
            pub fn set_field(&mut self, name: &str, raw: &str) -> Result<(), SetFieldError> {
                match name {
                    $(
                        stringify!($field) => {
                            self.$field.value = raw.parse::<$ty>().map_err(|_| {
                                SetFieldError::InvalidValue(name.to_string(), raw.to_string())
                            })?;
                        },
                    )*
                    other => return Err(SetFieldError::UnknownOption(other.to_string())),
                }
                Ok(())
            }

            pub fn field_names() -> &'static [&'static str] {
                &[$( stringify!($field) ),*]
            }
        }
    };
}

define_server! {
    version: 4;

    allow_emote_fix: bool = true, "Enable auto emote replacement features";
    allow_embed_fix: bool = true, "Enable auto twitter/pixiv/etc embedding features";

    allow_markov_learning: bool = true, "Allow the bot to learn from messages in the guild";
    allow_markov_generation: bool = true, "Allow the bot to generate messages in the guild";
    allow_markov_random: bool = true, "Allow the bot to randomly generate messages in the guild";

    allow_word_counter: bool = true, "Allow the word counter to count words in the guild";

}

#[derive(sqlx::FromRow, Debug, Clone)]
#[allow(dead_code)]
pub struct Configuration {
    pub guild_id: i64,
    pub version: i64,
    pub server: Json<Server>,
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            guild_id: 0,
            version: Server::VERSION,
            server: Json(Server::default()),
        }
    }
}
