#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_must_use)]
#![allow(unused_assignments)]
#![windows_subsystem = "windows"]
use dark_light::*;
use druid::{AppLauncher, WindowDesc};
use image::*;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

mod files;
use files::*;

mod events;
mod types;

mod image_container;
use image_container::*;

mod image_widget;
use image_widget::*;

mod container;
use container::*;

mod toolbar_data;
use toolbar_data::*;

mod toolbar_widget;
use toolbar_widget::*;

mod button_data;
use button_data::*;

mod button_widget;
use button_widget::*;

mod app_state;
use app_state::*;

mod ui_builder;
use ui_builder::*;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Set the name of the file to load from the command line args, if they exist
    let mut image_receiver;
    let mut file_name = String::new();
    let mut files: Vec<PathBuf> = Vec::new();
    let mut current_index: usize = 0;
    let current_name: String;
    if args.len() > 1 {
        file_name = args[1].clone();
        image_receiver = AsyncImageLoader::new_from_string(&file_name);
        // Load the image in the background while we set up the UI
        image_receiver.load_image();
        // Make list of other files in current directory
        let file_path = Path::new(&file_name).canonicalize().unwrap();
        let current_folder = file_path.parent().unwrap();

        for entry in current_folder
            .read_dir()
            .expect("read_dir call failed")
            .flatten()
        {
            // TODO: Case insensitivity
            if (entry.path().extension() == Some(OsStr::new("jpg")))
                | (entry.path().extension() == Some(OsStr::new("jpeg")))
                | (entry.path().extension() == Some(OsStr::new("JPG")))
                | (entry.path().extension() == Some(OsStr::new("JPEG")))
                | (entry.path().extension() == Some(OsStr::new("png")))
                | (entry.path().extension() == Some(OsStr::new("PNG")))
            {
                files.push(entry.path());
            }
        }

        // Find & save index of the initial file
        for (index, entry) in files.iter().enumerate() {
            if entry.file_name() == file_path.file_name() {
                current_index = index;
                break;
            }
        }
        current_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
    } else {
        let image_bytes = include_bytes!("../resources/bananirb.jpg");
        let mut current_image = image::load_from_memory(image_bytes).unwrap();
        image_receiver = AsyncImageLoader::new_from_bytes(current_image);
        current_name = String::from("Bananna Birb.jpeg2000");
    }

    // Build the UI structure
    let main_window = WindowDesc::new(build_ui())
        .title("")
        .window_size((640., 480.));

    //Set initial state
    let theme_state = match dark_light::detect() {
        dark_light::Mode::Dark => true,
        dark_light::Mode::Light => false,
    };
    let mut initial_state = AppState::new(theme_state);
    initial_state.set_image_handler(Arc::new(Mutex::new(image_receiver)));
    initial_state.set_current_image();
    initial_state
        .set_current_image_name(current_name);
    initial_state.set_image_list(current_index, files);

    // Launch program
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Error: error.");
}
