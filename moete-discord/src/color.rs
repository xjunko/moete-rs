use rand::Rng;

use crate::serenity;

/// Returns random color
pub fn get_random_color() -> serenity::Color {
    let mut rng = rand::rng();
    let r: u8 = rng.random_range(0..=255);
    let g: u8 = rng.random_range(0..=255);
    let b: u8 = rng.random_range(0..=255);
    serenity::Color::from_rgb(r, g, b)
}

/// Converts a hex string to a serenity Color
/// ie: #ffffff -> serenity::Color::from_rgb(255, 255, 255)
pub fn from_string(hex: &str) -> Option<serenity::Color> {
    let hex = hex.trim_start_matches("#");
    if hex.len() != 6 {
        return None;
    }
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    Some(serenity::Color::from_rgb(r, g, b))
}
