use rand::{Rng, SeedableRng, rngs::StdRng};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

use crate::commands;
use crate::serenity;

pub async fn status_rotate(ctx: Arc<serenity::Context>, config: Arc<moete_core::Config>) {
    let mut rng = StdRng::from_seed([0; 32]);
    let status = config.get_status();

    loop {
        let i: usize = rng.random_range(0..status.len());
        let (activity, name) = status[i].split_once('|').unwrap_or(("W", "the world"));
        let activity_data = match activity {
            "W" => serenity::gateway::ActivityData::watching(name),
            "L" => serenity::gateway::ActivityData::listening(name),
            "P" => serenity::gateway::ActivityData::playing(name),
            _ => serenity::gateway::ActivityData::playing(name),
        };
        ctx.set_activity(Some(activity_data));
        sleep(Duration::from_secs(60 * 5)).await; // n minutes
    }
}

pub async fn memory_trim() {
    loop {
        unsafe {
            libc::malloc_trim(0);
        }
        sleep(Duration::from_secs(30)).await;
    }
}

pub async fn start(ctx: Arc<serenity::Context>, config: Arc<moete_core::Config>) {
    tokio::spawn(status_rotate(Arc::clone(&ctx), Arc::clone(&config)));
    tokio::spawn(memory_trim());
    tokio::spawn(commands::pakb::banner_rotate(
        Arc::clone(&ctx),
        Arc::clone(&config),
    ));
}
