use moete_core::MoeteContext;
use moete_core::errors::music::{
    ERR_SONGBIRD_NOT_INITIALIZED,
    RESP_NOTHING_PLAYING,
};

/// Stops the currently playing track, if any.
#[poise::command(prefix_command, slash_command, category = "Music")]
pub async fn stop(ctx: MoeteContext<'_>) -> Result<(), moete_core::MoeteError> {
    let guild_id =
        ctx.guild_id().ok_or("This command must be used in a guild")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(ERR_SONGBIRD_NOT_INITIALIZED)?
        .clone();

    let Some(call) = manager.get(guild_id) else {
        ctx.say(RESP_NOTHING_PLAYING).await?;
        return Ok(());
    };

    {
        let mut call_lock = call.lock().await;
        call_lock.stop();
    }

    ctx.say("Stopped.").await?;

    Ok(())
}
