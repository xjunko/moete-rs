use std::sync::Arc;
use tokio::time::sleep;

use crate::serenity;

/// Refreshes the emote list every n minutes
/// This is run as a background task
pub async fn refresh(ctx: Arc<serenity::Context>, state: moete_core::State) {
    loop {
        sleep(std::time::Duration::from_secs(60 * 60)).await; // n minutes
        {
            let mut emotes = state.emotes.lock().await;
            emotes.refresh(&ctx, state.config.clone()).await;
        }
    }
}
