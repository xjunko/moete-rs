use std::time::Instant;

use crate::builtins;
use crate::{Context, Error};

/// Replies with a message and get the difference in milliseconds.
#[poise::command(prefix_command, category = "Utility")]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = Instant::now();
    let msg = ctx.say("uwu").await?;
    let elapsed_ms = start.elapsed().as_millis();

    let embed_response = builtins::discord::embed::create_embed()
        .title("Pong!")
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
