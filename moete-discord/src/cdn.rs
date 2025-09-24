use base64::{Engine as _, engine::general_purpose};
use serde_json::json;
use std::sync::Arc;

pub async fn get_cdn_url(url: &str, config: Arc<moete_core::Config>) -> Option<String> {
    let client = reqwest::Client::new();

    if let Ok(res) = client
        .post("https://discord.com/api/v9/attachments/refresh-urls")
        .header("Authorization", &config.discord.token_cdn)
        .json(&json!({
            "attachment_urls": [url]
        }))
        .send()
        .await
    {
        if !res.status().is_success() {
            return None;
        }

        if let Ok(data) = res.json::<serde_json::Value>().await
            && let Some(link) = data["refreshed_urls"][0]["refreshed"].as_str()
        {
            return Some(link.to_string());
        }
    }

    None
}

pub async fn to_base64(url: &str) -> Option<String> {
    let client = reqwest::Client::new();

    if let Ok(res) = client.get(url).send().await {
        if !res.status().is_success() {
            return None;
        }

        if let Ok(bytes) = res.bytes().await {
            return Some(general_purpose::STANDARD.encode(bytes));
        }
    }

    None
}
