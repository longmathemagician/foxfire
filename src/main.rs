use std::env;
use std::sync::mpsc;
use std::thread;

use druid::AppLauncher;
use druid::LocalizedString;
use druid::WindowDesc;

// Import modules
mod types;
mod events;
mod view_widget;


#[windows_subsystem = "windows"] // Don't show the console at launch on windows
fn main() {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();

    // Set the name of the file to load from the command line args, if they exist
    let file_name = if args.len()>1 {args[1].clone()} else {String::from("/home/steve/Projects/foxfire/1.jpg")};

    // Send the name of the file to the thread that will load the image in the background
    let (tx_name, rx_name) = mpsc::channel();
    tx_name.send(file_name.clone()).unwrap();

    // Spawn a thread, load the image, and pass it back
    let (tx_data, rx_data) = mpsc::channel();
    thread::spawn(move || {
        let name = rx_name.recv().unwrap();
        let file_path = std::path::Path::new(&name);

        let img = image::open(file_path).unwrap();
        tx_data.send(img).unwrap();
    });

    // Create a window and a widget to display the image, passing it the receiver for the image along with initial data
    let window = WindowDesc::new(
        view_widget::ImageView::new(file_name.clone(),rx_data))
        .title(LocalizedString::new("Linux Photo Viewer"))
        .window_size((640., 480.));

    // Show the window
    AppLauncher::with_window(window)
        .log_to_console()
        .launch("launch string".to_string())
        .expect("launch failed");
}
