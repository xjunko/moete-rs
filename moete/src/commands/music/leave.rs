use moete_core::MoeteContext;
use moete_core::errors::music::{
    ERR_BOT_NOT_IN_VOICE_CHANNEL,
    ERR_FAILED_TO_LEAVE,
    ERR_SONGBIRD_NOT_INITIALIZED,
    RESP_LEFT_VC,
};

/// Leaves the voice channel the bot is currently in.
#[poise::command(prefix_command, slash_command, category = "Music")]
pub async fn leave(
    ctx: MoeteContext<'_>,
) -> Result<(), moete_core::MoeteError> {
    let guild_id =
        ctx.guild_id().ok_or("This command must be used in a guild")?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(ERR_SONGBIRD_NOT_INITIALIZED)?
        .clone();

    ctx.defer().await?;

    if manager.get(guild_id).is_none() {
        ctx.say(ERR_BOT_NOT_IN_VOICE_CHANNEL).await?;
        return Ok(());
    }

    match manager.remove(guild_id).await {
        Ok(()) => {
            ctx.say(RESP_LEFT_VC).await?;
        },

        Err(err) => {
            ctx.say(format!("{ERR_FAILED_TO_LEAVE}: {err}")).await?;
        },
    }

    // music libraries is fairly expensive
    moete_core::memory::trim_memory();

    Ok(())
}
