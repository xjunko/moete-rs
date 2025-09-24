use poise::serenity_prelude as serenity;
use serenity::all::CreateEmbedFooter;

use crate::color;

/// Creates a basic moete-branded embed.
pub fn create_embed() -> serenity::CreateEmbed {
    serenity::CreateEmbed::default()
        .color(color::get_random_color())
        .footer(CreateEmbedFooter::new(moete_core::version()))
}
