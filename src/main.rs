use bevy::prelude::*;
use bevy_egui::egui::{Align, Button, Color32, Frame, Layout, Stroke, TextEdit, TextStyle};
use bevy_egui::{egui, EguiContext, EguiPlugin};

// USE
use crate::egui::RichText;
use crate::fonts::setup_fonts;

// MODULES
mod fonts;

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
        .insert_resource(InputField {
            text: "".to_string(),
        })
        // PLUGINS
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // SYSTEMS
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

struct InputField {
    text: String,
}

fn move_to_next_state(mut app_state: ResMut<State<AppState>>) {
    let next_state = AppState::next(*app_state.current());
    app_state.set(next_state).unwrap();
}

fn show_ui(mut input_text: ResMut<InputField>, mut ctx: ResMut<EguiContext>) {
    let mut start_typing = false;

    egui::SidePanel::left("left_panel")
        .default_width(200.)
        .resizable(false)
        .frame(Frame {
            margin: egui::Vec2::new(5., 200.),
            stroke: Stroke::new(3., Color32::KHAKI),
            ..Default::default()
        })
        .show(ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                if ui
                    .button(RichText::new("START").color(Color32::LIGHT_YELLOW))
                    .clicked()
                {
                    start_typing = true;
                }
                if ui.button("SCORES").clicked() {}
                let hover_button =
                    ui.add(Button::new("HOVER").stroke(Stroke::new(2., Color32::LIGHT_RED)));
                if hover_button.clicked() {}
                if hover_button.hovered() {
                    info!("HOVERED");
                }
            });
        });

    egui::CentralPanel::default()
        .frame(Frame {
            margin: egui::Vec2::new(200., 100.),
            ..Default::default()
        })
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
