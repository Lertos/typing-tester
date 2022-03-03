use crate::colors;
use bevy_egui::egui::{Color32, Visuals};

pub struct Theme {
    visuals: Visuals,
}

impl Theme {
    pub fn new() -> Self {
        let mut visuals = Visuals::dark();

        visuals.widgets.noninteractive.bg_fill = colors::GENERAL_BACKGROUND_COLOR;
        visuals.widgets.noninteractive.fg_stroke.color = Color32::WHITE;
        visuals.widgets.noninteractive.corner_radius = 0.;

        visuals.widgets.inactive.bg_fill = colors::BUTTON_BACKGROUND_COLOR;
        visuals.widgets.hovered.bg_fill = colors::BUTTON_HOVERED_BACKGROUND_COLOR;
        visuals.widgets.active.bg_fill = colors::BUTTON_ACTIVE_BACKGROUND_COLOR;

        Self { visuals }
    }

    pub fn visuals(&self) -> &Visuals {
        &self.visuals
    }
}
