use super::serenity;
use rand::{Rng, SeedableRng, rngs::StdRng};
use std::{sync::Arc, time::Duration};
use tokio::time::sleep;

pub async fn status_rotate(ctx: Arc<serenity::Context>, status: Vec<String>) {
    let mut rng = StdRng::from_seed([0; 32]);
    loop {
        let i: usize = rng.random_range(0..status.len());
        ctx.set_activity(Some(serenity::gateway::ActivityData::watching(
            status.get(i).unwrap(),
        )));
        sleep(Duration::from_secs(60 * 5)).await; // n minutes
    }
}

pub async fn start(ctx: Arc<serenity::Context>, status: Vec<String>) {
    tokio::spawn(status_rotate(Arc::clone(&ctx), status));
}
