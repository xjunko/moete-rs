use std::sync::Arc;

use chrono::{Datelike, Local};
use serenity::all::{CreateAttachment, EditGuild, GuildId};
use tracing::info;

use super::data::SERVER_THEME;
use crate::serenity;

const PAKB: GuildId = GuildId::new(696405704134885466);
const INTERVAL: u64 = 3600 * 6; // 6 hours

pub async fn banner_rotate(ctx: Arc<serenity::Context>, config: Arc<moete_core::Config>) {
    loop {
        let custom_offset: i32 = 10;
        let current_day_of_year: i32 = Local::now().ordinal() as i32;
        let current_index: i32 = (current_day_of_year + custom_offset) % SERVER_THEME.len() as i32;

        let data = SERVER_THEME[current_index as usize];
        let (name, logo, banner) = (data[0], data[1], data[2]);

        let icon_attachment = {
            if let Some(logo_url) = moete_discord::cdn::get_cdn_url(logo, config.clone()).await {
                CreateAttachment::url(&ctx.http, &logo_url).await.ok()
            } else {
                None
            }
        };

        let banner_attachment: Option<String> = {
            if let Some(banner_url) = moete_discord::cdn::get_cdn_url(banner, config.clone()).await
            {
                moete_discord::cdn::to_base64(&banner_url).await
            } else {
                None
            }
        };

        if icon_attachment.is_none() || banner_attachment.is_none() {
            info!("Failed to fetch server banner or icon, skipping update");
            tokio::time::sleep(tokio::time::Duration::from_secs(INTERVAL)).await;
            continue;
        }

        match PAKB
            .edit(
                &ctx.http,
                EditGuild::new()
                    .name(name)
                    .icon(icon_attachment.as_ref())
                    .banner(banner_attachment),
            )
            .await
        {
            Ok(_) => info!("Updated server banner to {}", name),
            Err(e) => info!("Failed to update server banner: {:?}", e),
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(INTERVAL)).await;
    }
}
