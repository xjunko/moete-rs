use std::sync::{Arc, RwLock};

use moete_core::{MoeteContext, MoeteError};
use poise::CreateReply;
use rhai::{
    Engine,
    packages::{BasicMathPackage, Package},
};

/// Creates a barebones Rhai engine.
fn create_engine() -> Engine {
    let mut engine = Engine::new();
    engine.set_max_strings_interned(256);
    engine.disable_symbol("while");

    // for math expressions
    let basic_math_pkg = BasicMathPackage::new();
    basic_math_pkg.register_into_engine(&mut engine);

    engine
}

fn clean_source_code(source: String) -> String {
    let mut cleaned = source.clone();

    // remove code blocks
    if cleaned.starts_with("```") {
        // start
        cleaned = cleaned.lines().skip(1).collect::<Vec<&str>>().join("\n");
    }

    if cleaned.ends_with("```") {
        // end
        cleaned = cleaned
            .lines()
            .take(cleaned.lines().count() - 1)
            .collect::<Vec<&str>>()
            .join("\n");
    }

    // if there's no print, its likely a very simple math eval
    if !cleaned.contains("print") {
        cleaned = format!("print({});", cleaned);
    }

    cleaned
}

/// Runs a snippet of Rhai code and returns the output.
#[poise::command(
    prefix_command,
    slash_command,
    category = "Math",
    aliases("calculate", "eval")
)]
pub async fn calc(
    ctx: MoeteContext<'_>,
    #[description = "Math expression to evaluate"]
    #[rest]
    code: String,
) -> Result<(), MoeteError> {
    let mut engine = create_engine();

    // logging
    let logs = Arc::new(RwLock::new(Vec::<String>::new()));
    {
        let log = logs.clone();
        engine.on_print(move |s| {
            log.write().unwrap().push(s.to_string());
        });
    }

    // interpret
    {
        let cleaned_code = clean_source_code(code.clone());
        engine.run(&cleaned_code)?;

        let output = {
            let result = logs.read().unwrap().join("\n");

            if result.is_empty() {
                "No output.".to_string()
            } else {
                result
            }
        };

        let data: &moete_core::State = ctx.data();
        let embed = moete_discord::embed::create_embed()
            .title(format!("{} | Evaluation Output", data.config.discord.name))
            .field("Input", format!("```{}```", cleaned_code), false)
            .field("Result", format!("```{}```", output), false);

        ctx.send(CreateReply::default().embed(embed).reply(true))
            .await?;

        // dont need output anymore
        std::mem::drop(output);
    }

    // clean everything else
    std::mem::drop(engine);
    std::mem::drop(logs);
    unsafe {
        libc::malloc_trim(0);
    }

    Ok(())
}
