use moete_core::MoeteContext;
use moete_core::errors::music::{
    ERR_FAILED_TO_JOIN,
    ERR_SONGBIRD_NOT_INITIALIZED,
    ERR_USER_NOT_IN_VOICE_CHANNEL,
    RESP_JOINED_VC,
};

/// Joins the user's current voice channel.
#[poise::command(prefix_command, slash_command, category = "Music")]
pub async fn join(ctx: MoeteContext<'_>) -> Result<(), moete_core::MoeteError> {
    let guild_id =
        ctx.guild_id().ok_or("This command must be used in a guild")?;

    let channel_id = ctx
        .guild()
        .and_then(|guild| {
            guild
                .voice_states
                .get(&ctx.author().id)
                .and_then(|state| state.channel_id)
        })
        .ok_or(ERR_USER_NOT_IN_VOICE_CHANNEL)?;

    let manager = songbird::get(ctx.serenity_context())
        .await
        .ok_or(ERR_SONGBIRD_NOT_INITIALIZED)?
        .clone();

    ctx.defer().await?;

    match manager.join(guild_id, channel_id).await {
        Ok(_) => {
            ctx.say(RESP_JOINED_VC).await?;
        },

        Err(err) => {
            ctx.say(format!("{ERR_FAILED_TO_JOIN}\nReason: {err}")).await?;
        },
    }

    Ok(())
}
