#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![windows_subsystem = "windows"]
use druid::{AppLauncher, WindowDesc};
use image::*;
use std::env;
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
    let mut file_name;
    if args.len() > 1 {
        file_name = args[1].clone();
        image_receiver = AsyncImageLoader::new_from_string(&file_name);
        // Load the image in the background while we set up the UI
        image_receiver.load_image();
    } else {
        file_name = String::from("/home/steve/Projects/foxfire/resources/bananirb.jpg");
        let image_bytes = include_bytes!("../resources/bananirb.jpg");
        let mut current_image = image::load_from_memory(image_bytes).unwrap();
        image_receiver = AsyncImageLoader::new_from_bytes(current_image);
    }

    let file_path = Path::new(&file_name);
    let current_folder = file_path.parent().unwrap();
    let mut files: Vec<PathBuf> = Vec::new();
    let mut current_index: usize = 0;

    for entry in current_folder.read_dir().expect("read_dir call failed") {
        if let Ok(entry) = entry {
            files.push(entry.path());
        }
    }

    for (index, entry) in files.iter().enumerate() {
        if entry.file_stem() == file_path.file_stem() {
            println!("DUPLICATE ENTRY AT INDEX {:#?}", index);
            current_index = index;
        } else {
            println!("{:?}", entry);
        }
    }

    let main_window = WindowDesc::new(build_ui())
        .title("")
        .window_size((640., 480.));

    let mut initial_state = AppState::new();
    initial_state.set_image_handler(Arc::new(Mutex::new(image_receiver)));
    initial_state.set_current_image();
    initial_state.set_image_list(current_index, files);

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Error: error.");
}
