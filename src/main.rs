#![windows_subsystem = "windows"]
use druid::{AppLauncher, WindowDesc};
use std::env;

mod files;

use files::*;

mod events;

mod types;

mod commands;

use commands::*;

mod button_data;
mod button_widget;
mod container;
mod display_image_container;
mod image_widget;
mod toolbar_data;
mod toolbar_widget;

mod app_state;
use app_state::*;

mod ui_builder;
use ui_builder::*;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    let file_name: String = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };

    // Build the UI structure
    let main_window = WindowDesc::new(build_ui())
        .title("")
        .with_min_size((450., 240.))
        .window_size((640., 480.));
    let launcher = AppLauncher::with_window(main_window).log_to_console();

    //Set initial state
    let theme_state = match dark_light::detect() {
        dark_light::Mode::Dark => true,
        dark_light::Mode::Light => false,
    };
    let mut initial_state = AppState::from(theme_state, launcher.get_external_handle());
    initial_state.startup(file_name);

    // Launch program
    launcher
        .delegate(Delegate::new())
        .launch(initial_state)
        .expect("Failed to launch application");
}
