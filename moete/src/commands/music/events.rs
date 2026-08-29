use std::sync::Arc;
use std::time::Duration;

use serenity::async_trait;
use serenity::builder::{
    CreateEmbed,
    EditMessage,
};
use serenity::http::Http;
use serenity::model::channel::Message;
use songbird::events::{
    Event,
    EventContext,
    EventHandler as VoiceEventHandler,
};
use songbird::input::AuxMetadata;
use tracing::error;

pub(crate) struct MoeteTrackOnError;

#[async_trait]
impl VoiceEventHandler for MoeteTrackOnError {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            for (state, handle) in *track_list {
                error!(
                    "Track {:?} errored: {:?}",
                    handle.uuid(),
                    state.playing
                );
            }
        }
        None
    }
}

pub(crate) struct MoeteTrackOnEnd {
    pub(crate) manager: Arc<songbird::Songbird>,
    pub(crate) guild_id: serenity::model::id::GuildId,
}

#[async_trait]
impl VoiceEventHandler for MoeteTrackOnEnd {
    async fn act(
        &self,
        _ctx: &songbird::EventContext<'_>,
    ) -> Option<songbird::Event> {
        // HACK: might want to wait for a little bit
        // since there might be a new track added...
        tokio::time::sleep(Duration::from_millis(2000)).await;

        if let Some(call) = self.manager.get(self.guild_id) {
            let call_lock = call.lock().await;
            // if a new track already got added (by play replacing this one), don't leave
            if call_lock.queue().current().is_some() {
                return None;
            }
        }

        if let Err(err) = self.manager.remove(self.guild_id).await {
            error!("Failed to auto-leave after track end: {err}");
        }
        None
    }
}

pub(crate) struct MoeteTrackOnUpdate {
    pub(crate) http: Arc<Http>,
    pub(crate) embed: CreateEmbed,
    pub(crate) metadata: AuxMetadata,
    pub(crate) total_length: Option<std::time::Duration>,
    pub(crate) status_msg: Option<Message>,
}

impl MoeteTrackOnUpdate {
    fn render_progress_bar(
        position: std::time::Duration,
        total: Option<std::time::Duration>,
    ) -> String {
        let Some(total) = total else {
            return format!(
                "`{}` (unknown length)",
                Self::format_duration(position)
            );
        };

        let pct =
            (position.as_millis() as f64 / total.as_millis() as f64).min(1.0);
        let filled = (pct * 20.0).round() as usize;

        let bar = "=".repeat(filled.saturating_sub(1))
            + ">"
            + &"-".repeat(20usize.saturating_sub(filled));

        format!(
            "`[{}] {} / {}`",
            bar,
            Self::format_duration(position),
            Self::format_duration(total)
        )
    }

    fn format_duration(d: std::time::Duration) -> String {
        let total = d.as_secs();
        let (h, m, s) = (total / 3600, (total % 3600) / 60, total % 60);
        if h > 0 { format!("{h}:{m:02}:{s:02}") } else { format!("{m}:{s:02}") }
    }
}

#[async_trait]
impl VoiceEventHandler for MoeteTrackOnUpdate {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx
            && let Some((state, _handle)) = track_list.first()
        {
            let position = state.position;
            let bar = Self::render_progress_bar(position, self.total_length);

            if let Some(mut status_msg) = self.status_msg.clone() {
                let mut embed = self
                    .embed
                    .clone()
                    .field(
                        "Information",
                        format!(
                            "**Title**: {}\n**Channel**: {}\n**Duration**: {}",
                            self.metadata
                                .title
                                .as_deref()
                                .unwrap_or("Unknown title"),
                            self.metadata
                                .channel
                                .as_deref()
                                .unwrap_or("Unknown channel"),
                            self.metadata
                                .duration
                                .map(Self::format_duration)
                                .unwrap_or_else(|| "Unknown duration".into())
                        ),
                        false,
                    )
                    .field("_ _", bar, false);

                if let Some(thumbnail) = self.metadata.thumbnail.as_deref() {
                    embed = embed.thumbnail(thumbnail);
                }

                if let Err(err) = status_msg
                    .edit(&self.http, EditMessage::new().embed(embed))
                    .await
                {
                    error!("Failed to update status message: {err}");
                }
            }
        }
        None
    }
}
