use crate::serenity;
use rand::Rng;

pub fn get_random_color() -> serenity::Color {
    let mut rng = rand::rng();
    let r: u8 = rng.random_range(0..=255);
    let g: u8 = rng.random_range(0..=255);
    let b: u8 = rng.random_range(0..=255);
    serenity::Color::from_rgb(r, g, b)
}
