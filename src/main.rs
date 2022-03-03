use bevy::prelude::*;
use bevy::window::WindowResizeConstraints;
use bevy_egui::egui::{Align, Layout, TextEdit, TextStyle, Widget};
use bevy_egui::{egui, EguiContext, EguiPlugin};

// USE
use crate::fonts::setup_fonts;
use crate::theme::Theme;
use crate::widgets::{
    InputField, StyledButton, StyledCentralPanel, StyledSidePanel, WindowForLabels,
};

// MODULES
mod colors;
mod fonts;
mod theme;
mod widgets;

// SETUP CONSTANTS
const MINIMUM_WINDOW_WIDTH: f32 = 800.;
const MINIMUM_WINDOW_HEIGHT: f32 = 500.;

const SPACE_BETWEEN_LABELS: f32 = 20.;

const INPUT_SIZE: egui::Vec2 = egui::Vec2::new(240., 60.);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InitialSetup,
    Setup,
    StartGame,
}

impl AppState {
    fn next(state: AppState) -> AppState {
        match state {
            AppState::InitialSetup => AppState::Setup,
            AppState::Setup => AppState::StartGame,
            AppState::StartGame => AppState::StartGame,
        }
    }
}

fn main() {
    let mut app = App::new();

    app.add_state(AppState::InitialSetup)
        // WINDOW CUSTOMIZATION
        .insert_resource(WindowDescriptor {
            title: "Typing Tester".to_string(),
            width: 1280.,
            height: 720.,
            position: Some(Vec2::new(0., 0.)),
            resize_constraints: WindowResizeConstraints {
                min_width: MINIMUM_WINDOW_WIDTH,
                min_height: MINIMUM_WINDOW_HEIGHT,
                ..Default::default()
            },
            ..Default::default()
        })
        // PLUGINS
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // SYSTEMS
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::on_enter(AppState::InitialSetup)
                .with_system(setup_fonts.label("initial_setup"))
                .with_system(move_to_next_state.after("initial_setup")),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::Setup).with_system(move_to_next_state.after("setup")),
        )
        .add_system_set(SystemSet::on_update(AppState::StartGame).with_system(show_ui))
        .run();
}

fn move_to_next_state(mut app_state: ResMut<State<AppState>>) {
    let next_state = AppState::next(*app_state.current());
    app_state.set(next_state).unwrap();
}

fn setup(mut commands: Commands, mut ctx: ResMut<EguiContext>) {
    ctx.ctx_mut().set_visuals(Theme::new().visuals().clone());

    commands.insert_resource(InputField {
        text: String::from("TYPE HERE"),
        enabled: false,
    })
}

fn show_ui(
    mut input_text: ResMut<InputField>,
    mut ctx: ResMut<EguiContext>,
    mut windows: ResMut<Windows>,
) {
    let input_enabled = input_text.enabled;

    let window = windows.get_primary_mut().unwrap();

    StyledSidePanel::new()
        .side_panel()
        .show(ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                //Can change the visuals multiple times throughout the painting of the UI
                //ui.visuals_mut().widgets.hovered.bg_fill = Color32::YELLOW;

                let button_start = StyledButton::new("START").ui(ui);
                if button_start.clicked() {
                    input_text.text = "".to_string();
                    input_text.enabled = true;
                }

                let button_scores = StyledButton::new("SCORES").ui(ui);
                if button_scores.clicked() {}

                let button_new = StyledButton::new("NEW").ui(ui);
                if button_new.clicked() {}
            });
        });

    StyledCentralPanel::new(window.width(), window.height())
        .central_panel()
        .show(ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let input = ui.add_sized(
                    INPUT_SIZE,
                    TextEdit::singleline(&mut input_text.text)
                        .text_style(TextStyle::Heading)
                        .interactive(input_enabled),
                );

                //If the start button was clicked, make sure focus is directed towards the input
                if input_enabled {
                    input.request_focus();
                }
                // If space is pressed, go to the next word
                if input.changed() && ui.input().key_pressed(egui::Key::Space) {
                    info!("SPACE PRESSED");
                }
                // Check if the letter typed is the correct next letter
                else if input.changed() {
                    info!("Response changed");
                }
                // To make sure the focus is always on the input
                if input.lost_focus() {
                    input.request_focus();
                }

                ui.add_space(60.);

                WindowForLabels::new().show(ui.ctx(), |ui| {
                    ui.add_space(SPACE_BETWEEN_LABELS);
                    ui.label("tiny chickens abstracted");
                    ui.add_space(SPACE_BETWEEN_LABELS);
                    ui.label("absorbed army responsible");
                    ui.add_space(SPACE_BETWEEN_LABELS);
                    ui.label("torpid afternoon defiant");
                    ui.add_space(SPACE_BETWEEN_LABELS);
                    ui.label("weak domineering park");
                    ui.add_space(SPACE_BETWEEN_LABELS);
                });
            });
        });
}
