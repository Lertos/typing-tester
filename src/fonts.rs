use bevy::prelude::*;
use bevy_egui::egui::FontData;
use bevy_egui::EguiContext;

use crate::egui::{FontDefinitions, FontFamily, TextStyle};

pub fn setup_fonts(mut ctx: ResMut<EguiContext>) {
    let mut fonts = FontDefinitions::default();

    fonts.font_data.insert(
        "toxigenesis".to_owned(),
        FontData::from_static(include_bytes!("../assets/Toxigenesis.ttf")),
    );

    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "toxigenesis".to_owned());

    fonts
        .family_and_size
        .insert(TextStyle::Small, (FontFamily::Monospace, 16.0));

    fonts
        .family_and_size
        .insert(TextStyle::Body, (FontFamily::Monospace, 26.0));

    fonts
        .family_and_size
        .insert(TextStyle::Heading, (FontFamily::Monospace, 38.0));

    fonts
        .family_and_size
        .insert(TextStyle::Button, (FontFamily::Monospace, 24.0));

    fonts
        .family_and_size
        .insert(TextStyle::Monospace, (FontFamily::Monospace, 20.0));

    ctx.ctx_mut().set_fonts(fonts);
}
