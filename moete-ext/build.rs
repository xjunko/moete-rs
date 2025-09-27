use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
struct MacroDef {
    #[serde(default)]
    aliases: Option<Vec<String>>,
    #[serde(deserialize_with = "string_or_vec")]
    reply: Vec<String>,
}
fn string_or_vec<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum OneOrMany {
        One(String),
        Many(Vec<String>),
    }

    match OneOrMany::deserialize(deserializer)? {
        OneOrMany::One(s) => Ok(vec![s]),
        OneOrMany::Many(v) => Ok(v),
    }
}

fn main() {
    if std::env::var("CARGO_FEATURE_MACROS").is_err() {
        println!("cargo:rerun-if-changed=files/commands.json");
        return;
    }

    let json_path = std::path::Path::new("files/commands.json");
    let json = std::fs::read_to_string(json_path)
        .expect(&format!("Failed to read moete's commands: {:?}", json_path).to_string());
    let data: HashMap<String, MacroDef> =
        serde_json::from_str(&json).expect("Failed to parse moete's commands JSON");

    // generate code
    let mut out = String::new();
    let mut commands: Vec<String> = Vec::new();

    out.push_str("use crate::{Context, Error};\n");
    out.push_str("use rand::Rng;\n");

    for (name, def) in data {
        let mut maybe_aliases = String::new();

        if def.aliases.is_some() {
            maybe_aliases.push_str(&format!(
                ", aliases({})",
                def.aliases
                    .unwrap()
                    .iter()
                    .map(|a| format!("\"{}\"", a))
                    .collect::<Vec<String>>()
                    .join(", ")
            ));
        }

        out.push_str(&format!(
            "#[cfg(feature = \"macros\")]\n\
            #[poise::command(prefix_command, slash_command, category = \"Fun\"{})]\n",
            maybe_aliases
        ));
        out.push_str(&format!(
            "pub async fn {}(ctx: Context<'_>) -> Result<(), Error> {{\n",
            name
        ));
        out.push_str("    let replies = vec![\n");
        for reply in def.reply {
            out.push_str(&format!("        r#\"{}\"#,\n", reply));
        }
        out.push_str("    ];\n");

        // random
        out.push_str("    let reply = replies.get(rand::rng().random_range(0..replies.len())).unwrap().to_string();\n");
        out.push_str("    ctx.say(reply).await?;\n");
        out.push_str("    Ok(())\n");
        out.push_str("}\n\n");

        //
        commands.push(name.clone());
    }

    out.push_str("pub fn macro_commands() -> Vec<poise::Command<moete_core::State, Error>> {\n");
    out.push_str("    vec![\n");
    for command in commands {
        out.push_str(&format!("    {}(),\n", command));
    }
    out.push_str("    ]\n}");

    let dest_path = std::path::Path::new("../moete/src/commands/macros").join("commands.rs");
    std::fs::write(&dest_path, out.clone()).expect("Failed to write macro_commands.rs");

    println!("cargo:rerun-if-changed=files/commands.json");
}
