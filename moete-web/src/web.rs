use std::sync::Arc;

use axum::response::Html;
use tokio::{net::TcpListener, sync::Mutex};
use tracing::info;

/// starts the web server for moete
pub async fn start(config: Arc<moete_core::Config>, emotes: Arc<Mutex<moete_core::EmoteManager>>) {
    let app = axum::Router::new().route(
        "/",
        axum::routing::get({
            let emotes = emotes.clone();
            move || async move {
                let mut emotes_html = String::new();
                for emote in emotes.lock().await.global() {
                    emotes_html.push_str(&format!(
                        r#"<div class="window emote-item" data-name="{name}" style="height: 72px;">
            <img src="https://cdn.discordapp.com/emojis/{id}.webp?size=64" alt="{name}">
            <code>{name}</code>
        </div>"#,
                        id = emote.id,
                        name = emote.name
                    ));
                }

                Html(format!(
                    include_str!("../templates/emotes.tmpl"),
                    emotes_html = emotes_html
                ))
            }
        }),
    );

    tokio::spawn(async move {
        info!("Starting web server on http://{}", config.services.web);
        axum::serve(TcpListener::bind(&config.services.web).await.unwrap(), app)
            .await
            .unwrap();
    });
}
