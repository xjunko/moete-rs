use crate::builtins::branding;
use crate::builtins::discord::{embed, paginate};
use crate::serenity::all::CreateEmbedFooter;
use crate::{Context, Error};
use crate::{core, serenity};
use serde::Deserialize;

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
    pub async fn get(term: &str) -> Result<Vec<Urban>, Error> {
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
            .into_iter()
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
    ctx: Context<'_>,
    #[description = "The term to search for"]
    #[rest]
    term: String,
) -> Result<(), Error> {
    match Urban::get(&term).await {
        Ok(urban_definitions) => {
            let data: &core::State = ctx.data();
            let total = urban_definitions.len();

            let pages: Vec<serenity::CreateEmbed> = urban_definitions
                .into_iter()
                .map(|def| {
                    format!(
                        "**Definition:**\n{}\n\n**Example:**\n{}",
                        def.definition, def.example
                    )
                })
                .into_iter()
                .enumerate()
                .map(|(i, desc)| {
                    embed::create_embed()
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
                                branding::version()
                            ))
                            .icon_url(ctx.author().face()),
                        )
                })
                .collect();

            paginate::paginate_embed(ctx, pages).await?;
        }
        Err(reason) => {
            ctx.say(format!("failed to get urban dictionary entry: {}", reason))
                .await?;
        }
    }

    Ok(())
}
