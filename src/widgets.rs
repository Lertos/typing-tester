use bevy_egui::egui::{Button, Color32, Frame, Response, RichText, SidePanel, Ui, Vec2, Widget, CentralPanel};

use crate::colors;

pub struct InputField {
    pub text: String,
}

pub struct StyledButton {
    text: String,
}

impl StyledButton {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for StyledButton {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.add_sized(
            [ui.available_width(), ui.available_width() / 2.],
            Button::new(RichText::new(&self.text)).fill(colors::BUTTON_BACKGROUND_COLOR),
        )
    }
}

pub struct StyledSidePanel {
    panel: SidePanel,
}

impl StyledSidePanel {
    //TODO: Add window width and height as params and calculate the margin
    pub fn new() -> Self {
        Self {
            panel: SidePanel::left("left_panel")
                .default_width(200.)
                .resizable(false)
                .frame(Frame {
                    margin: Vec2::new(5., 200.), //TODO: Change based on window height
                    fill: Color32::BROWN,
                    ..Default::default()
                }),
        }
    }

    pub fn side_panel(self) -> SidePanel {
        self.panel
    }
}

pub struct StyledCentralPanel {
    panel: CentralPanel,
}

impl StyledCentralPanel {
    //TODO: Add window width and height as params and calculate the margin
    pub fn new() -> Self {
        Self {
            panel: CentralPanel::default()
            .frame(Frame {
                margin: Vec2::new(200., 100.), //TODO: Change based on window height
                ..Default::default()
            })
        }
    }

    pub fn central_panel(self) -> CentralPanel {
        self.panel
    }
}
