use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;

use druid::commands::SHOW_OPEN_PANEL;
use druid::{
    Application, ClipboardFormat, Data, ExtEventSink, FileDialogOptions, FileSpec, SingleUse,
    Target, WindowId,
};
use image::ImageOutputFormat;

use crate::display_image_container::*;
use crate::types::{Direction, NewImageContainer};
use crate::{IMAGE_LOADED, IMAGE_LOADING_STATE};

#[derive(Clone, Data)]
pub struct AppState {
    #[data(ignore)]
    window_id: Option<WindowId>,
    current_image: Arc<Mutex<DisplayImageContainer>>,
    has_image: bool,
    loading_new_image: Arc<Mutex<bool>>,
    current_image_index: usize,
    current_image_name: String,
    image_recenter_required: bool,
    image_list: Arc<Vec<PathBuf>>,
    druid_event_sink: Arc<Mutex<ExtEventSink>>,
    pub dark_theme_enabled: bool,
}

impl AppState {
    pub fn set_window_id(&mut self, id: WindowId) {
        self.window_id = Some(id);
    }

    pub fn from(dark_theme_enabled: bool, event_sink: ExtEventSink) -> Self {
        Self {
            window_id: None,
            current_image: Arc::new(Mutex::new(DisplayImageContainer::new())),
            has_image: false,
            loading_new_image: Arc::new(Mutex::new(false)),
            current_image_index: 0,
            current_image_name: String::new(),
            image_recenter_required: false,
            image_list: Arc::new(Vec::new()),
            druid_event_sink: Arc::new(Mutex::new(event_sink)),
            dark_theme_enabled,
        }
    }

    pub fn has_image(&self) -> bool {
        self.has_image
    }
    pub fn get_image_center_state(&self) -> bool {
        self.image_recenter_required
    }
    pub fn set_image_center_state(&mut self, state: bool) {
        self.image_recenter_required = state;
    }
    pub fn set_image_list(&mut self, index: usize, list: Vec<PathBuf>) {
        self.current_image_index = index;
        self.image_list = Arc::new(list);
    }

    pub fn startup(&mut self, path: String) {
        let file_path_result = Path::new(&path).canonicalize();
        if let Ok(file_path) = file_path_result {
            if file_path.is_file() {
                self.set_loading_state(true);
                self.load_image(&file_path);
                self.parse_folder(&file_path);
            } else if file_path.is_dir() {
                self.parse_folder(&file_path);
                let first_image = self.image_list[0].clone();
                self.load_image(&first_image);
            }
        } else {
            self.set_loading_state(false);
        }
    }

    fn parse_folder(&mut self, path: &Path) {
        let path_anchor = path.to_path_buf();

        let mut files: Vec<PathBuf> = Vec::new();
        let mut current_index: usize = 0;
        let current_file_name = path_anchor.file_name();
        let current_folder = path_anchor.parent().unwrap();

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
        if let Some(file_name) = current_file_name {
            for (index, entry) in files.iter().enumerate() {
                if let Some(entry_file_name) = entry.file_name() {
                    if entry_file_name == file_name {
                        current_index = index;
                        break;
                    }
                }
            }
        }

        // Set the image index and file list
        self.set_image_list(current_index, files);
    }

    fn load_image(&mut self, image_path: &Path) {
        let event_sink_mutex_ref = self.druid_event_sink.clone();
        let path_anchor = image_path.to_path_buf();
        thread::spawn(move || {
            let image_result = image::open(&path_anchor);
            let event_sink_mutex = event_sink_mutex_ref.lock().unwrap();
            let event_sink = &*event_sink_mutex;
            if let Ok(image) = image_result {
                let pth = path_anchor.to_str().unwrap().to_string();
                let wrapper = NewImageContainer::from_string_and_dynamicimage(pth, image);
                event_sink
                    .submit_command(IMAGE_LOADED, SingleUse::new(wrapper), Target::Auto)
                    .expect("Failed to send new image loaded command");
            } else {
                event_sink
                    .submit_command(IMAGE_LOADING_STATE, false, Target::Auto)
                    .expect("Failed to submit image loading failure notification command");
            }
        });
    }
    pub fn set_current_image(&mut self, container_wrapper: Option<NewImageContainer>) {
        if let Some(wrapper) = container_wrapper {
            {
                let mut current_image = self.current_image.lock().unwrap();
                current_image.set_image(wrapper.image);
            }
            self.set_current_image_name(wrapper.path);
            self.image_recenter_required = true;
            self.has_image = true;
        }
    }
    pub fn get_image_ref(&self) -> Arc<Mutex<DisplayImageContainer>> {
        self.current_image.clone()
    }
    pub fn get_toolbar_height(&self) -> f64 {
        80.0
    }
    pub fn load_next_image(&mut self) {
        if self.image_list.len() > 0 {
            println!(
                "Next image requested. Current index {}/{}",
                self.current_image_index,
                self.image_list.len()
            );
            if self.current_image_index >= self.image_list.len() - 1 {
                println!("Current image is last in folder, wrapping");
                self.current_image_index = 0;
            } else {
                self.current_image_index += 1;
            }
            self.current_image_name = self.image_list[self.current_image_index]
                .clone()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            print!("Current image name: {}", self.current_image_name);
            println!(
                "Next image requested. Current index {}/{}",
                self.current_image_index,
                self.image_list.len()
            );
            let next_image_path = self.image_list[self.current_image_index].clone();
            self.load_image(&next_image_path);
        }
    }
    pub fn load_prev_image(&mut self) {
        if self.image_list.len() > 0 {
            println!(
                "Previous image requested. Current index {}/{}",
                self.current_image_index,
                self.image_list.len()
            );
            if self.current_image_index == 0 {
                println!("Current image is first in folder, wrapping to last");
                self.current_image_index = self.image_list.len() - 1;
            } else {
                self.current_image_index -= 1;
            }
            self.current_image_name = self.image_list[self.current_image_index]
                .clone()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            print!("Current image name: {}. ", self.current_image_name);
            println!(
                "Previous image requested. Current index {}/{}",
                self.current_image_index,
                self.image_list.len()
            );
            let previous_image_path = self.image_list[self.current_image_index].clone();
            self.load_image(&previous_image_path);
        }
    }

    pub fn get_image_name(&self) -> String {
        self.current_image_name.clone()
    }

    pub fn set_current_image_name(&mut self, name: String) {
        self.current_image_name = name;
    }

    pub fn set_as_wallpaper(&self) {
        let mode_set_result = wallpaper::set_mode(wallpaper::Mode::Span);
        if mode_set_result.is_err() {
            println!("Could not set wallpaper mode");
        }

        let wallpaper_set_result = wallpaper::set_from_path(
            self.image_list[self.current_image_index]
                .clone()
                .to_str()
                .unwrap(),
        );
        if wallpaper_set_result.is_err() {
            println!("Could not set wallpaper")
        }
    }

    pub fn rotate_in_memory(&mut self, direction: Direction) {
        let mut image_container_mutex = self.current_image.lock().unwrap();
        let current_image_option = image_container_mutex.get_image();

        if let Some(image) = current_image_option {
            let rotated_image = match direction {
                Direction::Left => image.rotate270(),
                Direction::Right => image.rotate90(),
            };
            image_container_mutex.set_image(rotated_image);
            self.image_recenter_required = true;
        }
    }

    pub fn open_folder(&self) {
        opener::open(
            &self.image_list[self.current_image_index]
                .clone()
                .parent()
                .unwrap()
                .as_os_str(),
        )
        .expect("Could not open image location.");
    }

    pub fn copy_image_to_clipboard(&self) {
        let mut clipboard = Application::global().clipboard();
        let image_container_mutex = self.current_image.lock().unwrap();
        let current_image_option = image_container_mutex.get_image();

        if let Some(image) = current_image_option {
            let mut clipboard_data_buffer = std::io::Cursor::new(Vec::new());
            image
                .write_to(&mut clipboard_data_buffer, ImageOutputFormat::Png)
                .expect("Error encoding image file to in-memory buffer");
            let clipboard_data = [ClipboardFormat::new(
                "image/png",
                clipboard_data_buffer.into_inner(),
            )];
            clipboard.put_formats(&clipboard_data);
        }
    }

    pub fn get_loading_state(&self) -> bool {
        let loading_state = self.loading_new_image.lock().unwrap();
        *loading_state
    }
    pub fn set_loading_state(&mut self, new_state: bool) {
        let mut loading_state = self.loading_new_image.lock().unwrap();
        *loading_state = new_state;
    }

    pub fn show_file_load_dialog(&mut self) {
        if let Some(window_id) = self.window_id {
            let jpg = FileSpec::new("Joint Photographic Experts Group", &["jpg", "jpeg"]);
            let png = FileSpec::new("Portable Network Graphics", &["png"]);
            let image_file_types = FileSpec::new("Other image files", &["jpg", "jpeg", "png"]);
            let options = FileDialogOptions::new()
                .allowed_types(vec![jpg, png, image_file_types])
                .name_label("Image")
                .title("Choose an image to load")
                .button_text("Load");

            let event_sink = self.druid_event_sink.lock().unwrap();
            event_sink
                .submit_command(SHOW_OPEN_PANEL, options, window_id)
                .expect("Failed to send command");
        }
    }

    pub fn close_current_image(&mut self) {
        self.set_image_list(0, Vec::new());
        let mut current_image = self.current_image.lock().unwrap();
        self.current_image_name = String::new();
        current_image.clear_image();
        self.image_recenter_required = false;
        self.has_image = false;
    }
}
