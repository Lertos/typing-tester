use bevy::prelude::*;
use bevy::window::WindowResizeConstraints;
use bevy_egui::egui::{Align, Color32, Label, Layout, RichText, TextEdit, TextStyle, Ui, Widget};
use bevy_egui::{egui, EguiContext, EguiPlugin};

// USE
use crate::fonts::setup_fonts;
use crate::theme::Theme;
use crate::widgets::{
    InputField, StyledButton, StyledCentralPanel, StyledSidePanel, WindowForLabels,
    CENTRAL_PANEL_CONTEXT_WIDTH,
};
use crate::word_generator::{AllWords, WordList};

// MODULES
mod colors;
mod fonts;
mod theme;
mod widgets;
mod word_generator;

// SETUP CONSTANTS
const MINIMUM_WINDOW_WIDTH: f32 = 800.;
const MINIMUM_WINDOW_HEIGHT: f32 = 600.;

const VERT_SPACE_BETWEEN_LABELS: f32 = 14.;
const HORZ_SPACE_BETWEEN_LABELS: f32 = 14.;

const INPUT_SIZE: egui::Vec2 = egui::Vec2::new(240., 60.);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    ReadyToPlay,
    Playing,
    GameOver,
    Scores,
    FAQ,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, StageLabel)]
enum Stage {
    DrawPanels,
    DrawWindows,
}

fn main() {
    let mut app = App::new();

    app.add_state(AppState::Menu)
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
        // STAGES
        .add_stage_after(
            CoreStage::Update,
            Stage::DrawPanels,
            SystemStage::parallel(),
        )
        .add_stage_after(
            Stage::DrawPanels,
            Stage::DrawWindows,
            SystemStage::parallel(),
        )
        // STARTUP SYSTEMS
        .add_startup_system(setup)
        .add_startup_system(setup_fonts)
        // SYSTEMS
        .add_system_to_stage(Stage::DrawPanels, draw_ui)
        //.add_system_to_stage(Stage::DrawWindows, draw_windows)
        .run();
}

fn setup(mut commands: Commands, mut ctx: ResMut<EguiContext>) {
    ctx.ctx_mut().set_visuals(Theme::new().visuals().clone());

    commands.insert_resource(InputField {
        text: String::from(""),
        enabled: false,
    });

    commands.insert_resource(AllWords::new());
}

fn draw_ui(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    mut input_text: ResMut<InputField>,
    word_list: Res<AllWords>,
    current_words: Option<ResMut<WordList>>,
    mut ctx: ResMut<EguiContext>,
    mut windows: ResMut<Windows>,
) {
    let input_enabled = input_text.enabled;
    let mut index_increased = false;

    let window = windows.get_primary_mut().unwrap();

    StyledSidePanel::new()
        .side_panel()
        .show(ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down_justified(Align::Center), |ui| {
                let button_start = StyledButton::new("START").ui(ui);
                if button_start.clicked() {
                    if app_state.current() != &AppState::ReadyToPlay {
                        app_state.set(AppState::ReadyToPlay).unwrap();
                        commands.insert_resource(WordList::new(word_list.all_words.clone()));
                    }
                    input_text.text = "".to_string();
                    input_text.enabled = true;
                }

                let button_scores = StyledButton::new("SCORES").ui(ui);
                if button_scores.clicked() {
                    if app_state.current() != &AppState::Scores {
                        app_state.set(AppState::Scores).unwrap();
                    }
                }

                let button_new = StyledButton::new("FAQ").ui(ui);
                if button_new.clicked() {
                    if app_state.current() != &AppState::FAQ {
                        app_state.set(AppState::FAQ).unwrap();
                    }
                }
            });
        });

    StyledCentralPanel::new(window.width())
        .central_panel()
        .show(ctx.ctx_mut(), |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                if app_state.current() == &AppState::Menu {
                    ui.heading("Press Start");
                    return;
                } else if app_state.current() == &AppState::Scores {
                    ui.heading("TODO: Scores");
                    return;
                } else if app_state.current() == &AppState::FAQ {
                    ui.heading("TODO: FAQ");
                    return;
                }

                if app_state.current() == &AppState::ReadyToPlay {
                    ui.heading("Type to Begin");
                } else {
                    ui.heading("");
                }
                ui.add_space(20.);

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

                if input.changed() && ui.input().key_pressed(egui::Key::Space) {
                    // If the game hasn't started - ignore spaces
                    if app_state.current() == &AppState::ReadyToPlay {
                        input_text.text = "".to_string();
                    }
                    // If space is pressed and the game has started; move to the next word
                    else if app_state.current() == &AppState::Playing {
                        index_increased = true;
                    }
                }
                // Check if the letter typed is the correct next letter
                else if input.changed() {
                    // Start the game
                    if app_state.current() == &AppState::ReadyToPlay {
                        app_state.set(AppState::Playing).unwrap();
                    }

                    if app_state.current() == &AppState::ReadyToPlay
                        || app_state.current() == &AppState::Playing
                    {
                        //Logic to check correct/incorrect of key pressed versus word
                    }
                }
                // To make sure the focus is always on the input
                if input.lost_focus() {
                    input.request_focus();
                }

                ui.add_space(60.);

                // If the word list for the specific game instance has been generated
                if let Some(mut words) = current_words {
                    if index_increased {
                        words.current_index += 1;
                    }

                    // Used to know where to position the window as windows float and default to 0, 0
                    let end_point = ui.label("");

                    let rows: usize = 4;
                    let words_per_row: usize = 3;

                    let mut available_line_widths = Vec::<f32>::new();

                    // This window is here to find the available line widths so we can center labels
                    WindowForLabels::new(
                        3000.0, //Arbitrary numbers off-screen
                        3000.0,
                    )
                    .show(ui.ctx(), |ui| {
                        // To make sure words consisting of many labels stay together
                        ui.style_mut().spacing.item_spacing.x = 0.;
                        ui.style_mut().spacing.window_padding.x = 0.;

                        for row in 0..rows {
                            ui.horizontal(|ui| {
                                for word_index in 0..words_per_row {
                                    let current_index = get_current_word_index(
                                        row,
                                        word_index,
                                        words.current_index,
                                        words_per_row,
                                    );
                                    let current_word = &words.list[current_index];

                                    add_word_to_ui(
                                        ui,
                                        words.current_index,
                                        current_index,
                                        &input_text.text,
                                        &current_word,
                                    );

                                    if word_index < words_per_row - 1 {
                                        ui.add_space(HORZ_SPACE_BETWEEN_LABELS);
                                    }
                                }
                                available_line_widths.push(ui.available_width());
                            });
                        }
                    });

                    // This window is visible window that shows the player the words they need to type
                    WindowForLabels::new(
                        end_point.rect.left() - (CENTRAL_PANEL_CONTEXT_WIDTH / 4.),
                        end_point.rect.top(),
                    )
                    .show(ui.ctx(), |ui| {
                        // To make sure words consisting of many labels stay together
                        ui.style_mut().spacing.item_spacing.x = 0.;
                        ui.style_mut().spacing.window_padding.x = 0.;

                        for row in 0..rows {
                            let unused_width = available_line_widths[row];

                            ui.add_space(VERT_SPACE_BETWEEN_LABELS);

                            ui.horizontal(|ui| {
                                ui.add_space(unused_width / 4.);

                                for word_index in 0..words_per_row {
                                    let current_index = get_current_word_index(
                                        row,
                                        word_index,
                                        words.current_index,
                                        words_per_row,
                                    );
                                    let current_word = &words.list[current_index];

                                    add_word_to_ui(
                                        ui,
                                        words.current_index,
                                        current_index,
                                        &input_text.text,
                                        &current_word,
                                    );

                                    if word_index < words_per_row - 1 {
                                        ui.add_space(HORZ_SPACE_BETWEEN_LABELS);
                                    }
                                }

                                ui.add_space(unused_width / 4.);
                            });
                        }

                        ui.add_space(VERT_SPACE_BETWEEN_LABELS);
                    });

                    //Clear the input field for the next round of typing
                    if index_increased {
                        input_text.text = "".to_string();
                    }
                }
            });
        });
}

fn get_current_word_index(
    row_index: usize,
    word_index: usize,
    current_index: usize,
    words_per_row: usize,
) -> usize {
    let current_row = (current_index as f32 / words_per_row as f32).floor();
    let word_list_index =
        (row_index * words_per_row) + word_index + (current_row as usize * words_per_row);
    return word_list_index as usize;
}

fn add_word_to_ui(
    ui: &mut Ui,
    current_index: usize,
    word_index: usize,
    current_input: &String,
    current_word: &String,
) {
    // If this isn't the current word being typed
    if current_index != word_index {
        ui.add(Label::new(
            RichText::new(&current_word[..]).color(Color32::WHITE),
        ));
    } else {
        // Check how far into the word we are and if they match
        let length_typed = current_input.len();
        let word_length = current_word.len();
        
        if length_typed < word_length {
            ui.add(Label::new(RichText::new(&current_word[..length_typed]).color(Color32::RED)));
            ui.add(Label::new(RichText::new(&current_word[length_typed..]).color(Color32::WHITE)));
        } else {
            ui.add(Label::new(RichText::new(&current_word[..]).color(Color32::RED)));
        }
    }
}
