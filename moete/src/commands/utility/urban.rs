use serde::Deserialize;
use serenity::all::CreateEmbedFooter;

use crate::serenity;
use moete_core::{MoeteContext, MoeteError};

#[derive(Deserialize)]
struct Urban {
    _author: String,
    _word: String,
    // permalink: String,
    definition: String,
    example: String,
    _like: i32,
    _dislike: i32,
}

impl Urban {
    pub async fn get(term: &str) -> Result<Vec<Urban>, MoeteError> {
        let url = format!(
            "https://api.urbandictionary.com/v0/define?term={}",
            term.replace(" ", "%20")
        );
        let resp = reqwest::get(&url)
            .await?
            .json::<serde_json::Value>()
            .await?;

        if resp["list"].as_array().unwrap().is_empty() {
            return Err("No results found".into());
        }

        let mut entry: Vec<Urban> = resp["list"]
            .as_array()
            .unwrap()
            .iter()
            .map(|e| Self {
                _author: e["author"].as_str().unwrap().to_string(),
                _word: e["word"].as_str().unwrap().to_string(),
                // permalink: e["permalink"].as_str().unwrap().to_string(),
                definition: e["definition"].as_str().unwrap().to_string(),
                example: e["example"].as_str().unwrap().to_string(),
                _like: e["thumbs_up"].as_i64().unwrap() as i32,
                _dislike: e["thumbs_down"].as_i64().unwrap() as i32,
            })
            .collect();

        entry.sort_by(|a, b| a._like.cmp(&b._like));

        Ok(entry)
    }
}

/// Finds a term on Urban Dictionary and returns the definition(s).
#[poise::command(prefix_command, category = "Utility")]
pub async fn urban(
    ctx: MoeteContext<'_>,
    #[description = "The term to search for"]
    #[rest]
    term: String,
) -> Result<(), MoeteError> {
    match Urban::get(&term).await {
        Ok(urban_definitions) => {
            let data: &moete_core::State = ctx.data();
            let total = urban_definitions.len();

            let pages: Vec<serenity::CreateEmbed> = urban_definitions
                .iter()
                .map(|def| {
                    format!(
                        "**Definition:**\n{}\n\n**Example:**\n{}",
                        def.definition, def.example
                    )
                })
                .enumerate()
                .map(|(i, desc)| {
                    moete_discord::embed::create_embed()
                        .title(format!(
                            "{} | {} [{}/{}]",
                            data.config.discord.name,
                            term,
                            i + 1,
                            total
                        ))
                        .description(desc)
                        .footer(
                            CreateEmbedFooter::new(format!(
                                "Requested by {} | {}",
                                ctx.author().name,
                                moete_core::version()
                            ))
                            .icon_url(ctx.author().face()),
                        )
                })
                .collect();

            moete_discord::paginate::paginate_embed(ctx, pages).await?;
        }
        Err(reason) => {
            ctx.say(format!("failed to get urban dictionary entry: {}", reason))
                .await?;
        }
    }

    Ok(())
}
