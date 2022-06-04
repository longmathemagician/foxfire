#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::env;
use std::path::Path;
use std::sync::{Arc, Mutex};
use druid::{AppLauncher, WindowDesc};
use image::*;

mod files;
use files::*;

mod types;
mod events;

mod image_container;
use image_container::*;

mod image_widget;
use image_widget::*;

mod container;
use container::*;

mod toolbar_widget;
use toolbar_widget::*;

mod data;
use data::*;

mod view;
use view::*;

fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Set the name of the file to load from the command line args, if they exist
    let mut image_receiver;
    if args.len() > 1 {
        let file_name = args[1].clone();
        image_receiver = AsyncImageLoader::new_from_string(file_name);
        // Load the image in the background while we set up the UI
        image_receiver.load_image();
    } else {
        let image_bytes = include_bytes!("../resources/bananirb.jpg");
        let mut current_image = image::load_from_memory(image_bytes).unwrap();
        image_receiver = AsyncImageLoader::new_from_bytes(current_image);
    }

    

    let main_window = WindowDesc::new(build_ui())
        .title("")
        .window_size((640., 480.));

    let mut initial_state = AppState::new();
    initial_state.set_image_handler(Arc::new(Mutex::new(image_receiver)));
    initial_state.set_current_image();

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Error: error.");
}