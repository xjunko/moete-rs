use std::time::Duration;

use moete_core::MoeteContext;
use moete_core::errors::music::{
    ERR_FETCH_METADATA_FAILED,
    ERR_SONGBIRD_NOT_INITIALIZED,
    ERR_USER_NOT_IN_VOICE_CHANNEL,
    RESP_LOADING,
    RESP_REGISTERING_EVENTS,
};
use poise::CreateReply;
use songbird::input::{
    self,
    Compose,
};

use crate::commands::music::events::{
    MoeteTrackOnEnd,
    MoeteTrackOnError,
    MoeteTrackOnUpdate,
};

/// Plays a song from a YouTube URL in the user's current voice channel.
/// One track at a time for now.
#[poise::command(prefix_command, slash_command, category = "Music")]
pub async fn play(
    ctx: MoeteContext<'_>,
    #[description = "The URL of the song to play"] url: String,
) -> Result<(), moete_core::MoeteError> {
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

    let embed = moete_discord::embed::create_embed()
        .title(format!("{} | Music ", ctx.data().config.discord.name));

    let status = ctx
        .send(CreateReply::default().embed(embed.clone().field(
            "Status",
            RESP_LOADING,
            false,
        )))
        .await?;

    let call = manager.join(guild_id, channel_id).await?;
    {
        call.lock().await.stop(); // stop any currently playing track
    }

    {
        status
            .edit(
                ctx,
                CreateReply::default().embed(embed.clone().field(
                    "Status",
                    RESP_LOADING,
                    false,
                )),
            )
            .await?;
    }

    let mut source = input::YoutubeDl::new(reqwest::Client::new(), url.clone())
        .user_args(vec!["-f".into(), "bestaudio[ext=webm]/bestaudio".into()]);

    let source_metadata = source
        .aux_metadata()
        .await
        .map_err(|e| format!("{ERR_FETCH_METADATA_FAILED}: {e}"))?;

    let track_handle = call.lock().await.play_input(source.into());

    // TODO: maybe move this to events::register(...)
    {
        {
            status
                .edit(
                    ctx,
                    CreateReply::default().embed(embed.clone().field(
                        "Status",
                        RESP_REGISTERING_EVENTS,
                        false,
                    )),
                )
                .await?;
        }

        track_handle.add_event(
            songbird::Event::Track(songbird::TrackEvent::Error),
            MoeteTrackOnError,
        )?;
        track_handle.add_event(
            songbird::Event::Track(songbird::TrackEvent::End),
            MoeteTrackOnEnd { manager: manager.clone(), guild_id },
        )?;
        track_handle.add_event(
            songbird::Event::Periodic(Duration::from_secs(5), None),
            MoeteTrackOnUpdate {
                http: ctx.serenity_context().http.clone(),
                embed: embed.clone(),
                metadata: source_metadata.clone(),
                total_length: source_metadata.duration,
                status_msg: Some(status.into_message().await?),
            },
        )?;
    }

    Ok(())
}
