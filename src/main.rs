use bevy::prelude::*;
use bevy_egui::egui::Color32;
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
        .insert_resource(TestLabel { label: "".to_string()})
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
            SystemSet::on_enter(AppState::Setup)
                .with_system(move_to_next_state.after("setup")),
        )
        .add_system_set(SystemSet::on_update(AppState::StartGame).with_system(show_ui))
        .run();
}

struct TestLabel {
    label: String
}

/*
First things to figure out:
4. get general design of different modes, layouts, etc.
3. get general design of screen
5. get some logic/calculations in place
*/

fn move_to_next_state(mut app_state: ResMut<State<AppState>>) {
    let next_state = AppState::next(*app_state.current());
    app_state.set(next_state).unwrap();
}

fn show_ui(mut _label: ResMut<TestLabel>, mut ctx: ResMut<EguiContext>) {
    
    egui::TopBottomPanel::top("top_panel").show(ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            if ui.button(RichText::new("HOME").color(Color32::LIGHT_YELLOW)).clicked() {
                info!("Clicked HOME");
            }
            if ui.button("TEST").clicked() {
                info!("Clicked TEST");
            }
        });
    });

    egui::SidePanel::left("side_panel").show(ctx.ctx_mut(), |ui| {
        ui.heading("The Side Panel");

        ui.horizontal(|ui| {
            ui.label("Name: ");
            ui.text_edit_singleline(&mut _label.label);
        });

        ui.label("Known By: ".to_owned() + &_label.label[..]);

        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            ui.horizontal(|ui| {
                ui.label("Some Footer");
            });
        });
    });

    egui::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.heading("The Central Panel");
    });
}