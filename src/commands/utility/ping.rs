use serenity::all::{Color, CreateEmbed};

use crate::{Context, Error};
use std::time::Instant;

/// Replies with a message and get the difference in milliseconds.
#[poise::command(prefix_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    let msg = ctx.say("uwu").await?;
    let elapsed_ms = start.elapsed().as_millis();

    let embed_response = CreateEmbed::default()
        .title("Pong!")
        .color(Color::DARK_GREEN)
        .description(format!("Time taken: {}ms", elapsed_ms));

    msg.edit(
        ctx,
        poise::CreateReply::default()
            .content("")
            .embed(embed_response),
    )
    .await?;
    Ok(())
}
