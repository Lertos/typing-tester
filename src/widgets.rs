use bevy_egui::egui::{
    self, Button, CentralPanel, Frame, Pos2, Response, RichText, SidePanel, Stroke, Ui, Vec2,
    Widget, Window,
};

use crate::colors;

const SIDE_PANEL_DEFAULT_WIDTH: f32 = 200.;
const SIDE_PANEL_TOP_MARGIN: f32 = 200.;
const SIDE_PANEL_SIDE_MARGIN: f32 = 10.;

pub const CENTRAL_PANEL_CONTEXT_WIDTH: f32 = 700.;
const CENTRAL_PANEL_CONTEXT_HEIGHT: f32 = 100.;

const BUTTON_WIDTH: f32 = SIDE_PANEL_DEFAULT_WIDTH;
const BUTTON_HEIGHT: f32 = SIDE_PANEL_DEFAULT_WIDTH / 4.;
const BUTTON_STROKE_WIDTH: f32 = 3.;
const BUTTON_SPACE_BETWEEN: f32 = 10.;

pub struct InputField {
    pub text: String,
    pub enabled: bool,
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
        ui.add_space(BUTTON_SPACE_BETWEEN);

        ui.add_sized(
            [BUTTON_WIDTH, BUTTON_HEIGHT],
            Button::new(RichText::new(&self.text)).stroke(Stroke::new(
                BUTTON_STROKE_WIDTH,
                colors::BUTTON_STROKE_COLOR,
            )),
        )
    }
}

pub struct WindowForLabels;

impl WindowForLabels {
    pub fn new(window_width: f32, mut xpos: f32, ypos: f32) -> Window<'static> {
        // To avoid the below calculation for the window that is only there for getting widths
        if xpos == 0. {
            xpos = (window_width / 2.) + SIDE_PANEL_DEFAULT_WIDTH + SIDE_PANEL_SIDE_MARGIN
                - (CENTRAL_PANEL_CONTEXT_WIDTH / 2.);
        }
        Window::new("")
            .id(egui::Id::new("window_for_labels"))
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .enabled(false)
            .default_size(Vec2::new(
                CENTRAL_PANEL_CONTEXT_WIDTH,
                CENTRAL_PANEL_CONTEXT_HEIGHT,
            ))
            .frame(Frame {
                margin: Vec2::new(0., 0.),
                stroke: Stroke::new(3., colors::BUTTON_STROKE_COLOR),
                fill: colors::BUTTON_BACKGROUND_COLOR,
                ..Default::default()
            })
            .current_pos(Pos2::new(xpos, ypos))
    }
}

pub struct StyledSidePanel {
    panel: SidePanel,
}

impl StyledSidePanel {
    pub fn new() -> Self {
        Self {
            panel: SidePanel::left("left_panel")
                .default_width(SIDE_PANEL_DEFAULT_WIDTH)
                .resizable(false)
                .frame(Frame {
                    margin: Vec2::new(SIDE_PANEL_SIDE_MARGIN, SIDE_PANEL_TOP_MARGIN),
                    fill: colors::BUTTON_MENU_BACKGROUND_COLOR,
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
    pub fn new(window_width: f32) -> Self {
        Self {
            panel: CentralPanel::default().frame(Frame {
                margin: Vec2::new(
                    (window_width - SIDE_PANEL_DEFAULT_WIDTH - CENTRAL_PANEL_CONTEXT_WIDTH) / 2.,
                    CENTRAL_PANEL_CONTEXT_HEIGHT,
                ),
                fill: colors::GENERAL_BACKGROUND_COLOR,
                ..Default::default()
            }),
        }
    }

    pub fn central_panel(self) -> CentralPanel {
        self.panel
    }
}
