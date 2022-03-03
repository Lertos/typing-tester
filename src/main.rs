use bevy::prelude::*;
use bevy_egui::egui::{Align, Button, Color32, Layout, Stroke, TextEdit, TextStyle, Widget};
use bevy_egui::{egui, EguiContext, EguiPlugin};

// USE
use crate::egui::RichText;
use crate::fonts::setup_fonts;
use crate::theme::Theme;
use crate::widgets::{InputField, StyledButton, StyledSidePanel, StyledCentralPanel};

// MODULES
mod colors;
mod fonts;
mod theme;
mod widgets;

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
            title: "TypingTester".to_string(),
            width: 1280.,
            height: 720.,
            position: Some(Vec2::new(0., 0.)),
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
        text: String::from(""),
    })
}

fn show_ui(mut input_text: ResMut<InputField>, mut ctx: ResMut<EguiContext>) {
    let mut start_typing = false;

    StyledSidePanel::new()
        .side_panel()
        .show(ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                
                //Can change the visuals multiple times throughout the painting of the UI
                ui.visuals_mut().widgets.hovered.bg_fill = Color32::YELLOW;

                let start_button = ui.add_sized(
                    [0.0, 180.0], //TODO - calculate this in resize func (minus margin)
                    Button::new(RichText::new("START").color(Color32::LIGHT_YELLOW)),
                );

                if start_button.clicked() {
                    start_typing = true;
                }

                //Change it here too
                ui.visuals_mut().widgets.hovered.bg_fill = Color32::GREEN;

                if ui.button("SCORES").clicked() {}
                let hover_button =
                    ui.add(Button::new("HOVER").stroke(Stroke::new(2., Color32::LIGHT_RED)));
                if hover_button.clicked() {}

                let new_button = StyledButton::new("NEW").ui(ui);
            });
        });

        StyledCentralPanel::new().central_panel()
        .show(ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.heading("The Central Panel");

                let response = ui.add_sized(
                    [240.0, 50.0],
                    TextEdit::singleline(&mut input_text.text).text_style(TextStyle::Heading),
                );

                //If the start button was clicked, make sure focus is directed towards the input
                if start_typing {
                    response.request_focus();
                }

                // Check if the letter typed is the correct next letter
                if response.changed() {
                    //info!("Response changed");
                }
                // To make sure the focus is always on the input
                //if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                if response.lost_focus() {
                    //info!("Lost focus");
                    response.request_focus();
                }

                ui.add_space(20.);

                ui.horizontal_wrapped(|ui| {
                    ui.label("tiny");
                    ui.label("chickens");
                    ui.label("abstracted");
                    ui.label("absorbed");
                    ui.label("army");
                    ui.label("responsible");
                    ui.label("torpid");
                    ui.label("afternoon");
                    ui.label("defiant");
                    ui.label("weak");
                    ui.label("domineering");
                    ui.label("park");
                    ui.label("cough");
                    ui.label("dramatic");
                    ui.label("seal");
                    ui.label("spotty");
                    ui.label("unique");
                    ui.label("afford");
                    ui.label("burst");
                });
            });
        });
}
