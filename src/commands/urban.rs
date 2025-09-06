use crate::{Context, Error};
use poise::samples::paginate;
use serde::Deserialize;

#[derive(Deserialize)]
struct Urban {
    author: String,
    word: String,
    // permalink: String,
    definition: String,
    example: String,
    like: i32,
    dislike: i32,
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

        let entry: Vec<Urban> = resp["list"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|e| Self {
                author: e["author"].as_str().unwrap().to_string(),
                word: e["word"].as_str().unwrap().to_string(),
                // permalink: e["permalink"].as_str().unwrap().to_string(),
                definition: e["definition"].as_str().unwrap().to_string(),
                example: e["example"].as_str().unwrap().to_string(),
                like: e["thumbs_up"].as_i64().unwrap() as i32,
                dislike: e["thumbs_down"].as_i64().unwrap() as i32,
            })
            .collect();

        Ok(entry)
    }
}

#[poise::command(prefix_command)]
pub async fn urban(ctx: Context<'_>, #[rest] term: String) -> Result<(), Error> {
    match Urban::get(&term).await {
        Ok(urban_definitions) => {
            let pages: Vec<String> = urban_definitions.into_iter().map(|def|format!(
                    "**{}**\n\n**Definition:**\n{}\n\n**Example:**\n{}\n\n👍 {} | 👎 {} | Author: {}",
                    def.word,
                    def.definition,
                    def.example,
                    def.like,
                    def.dislike,
                    def.author
                )).collect();
            let response_refs: Vec<&str> = pages.iter().map(String::as_str).collect();
            paginate(ctx, &response_refs).await?;
        }
        Err(reason) => {
            ctx.say(format!("failed to get urban dictionary entry: {}", reason))
                .await?;
        }
    }

    Ok(())
}
