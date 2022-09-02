use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use druid::commands::SHOW_OPEN_PANEL;
use druid::{
    Application, ClipboardFormat, Command, Data, ExtEventSink, FileDialogOptions, FileSpec,
    SingleUse, Target, WindowId,
};
use image::{DynamicImage, ImageOutputFormat};
use preferences::{AppInfo, Preferences, PreferencesMap};

use crate::image_container::*;
use crate::types::{Direction, NewImageContainer};
use crate::{IMAGE_LOAD_FAILURE, IMAGE_LOAD_SUCCESS, IMAGE_ROTATION_COMPLETE, REDRAW_IMAGE};

const APP_SIG: AppInfo = AppInfo {
    name: env!("CARGO_PKG_NAME"),
    author: env!("CARGO_PKG_AUTHORS"),
};

#[derive(Clone, Data)]
pub struct AppState {
    #[data(ignore)]
    window_id: Option<WindowId>,
    #[data(ignore)]
    current_image: Arc<Mutex<ImageState>>,
    command_queue: Arc<Mutex<Vec<Command>>>,
    loading_new_image: Arc<Mutex<bool>>,
    rotating_image: Arc<Mutex<bool>>,
    current_image_index: usize,
    current_image_name: String,
    image_recenter_required: bool,
    image_list: Arc<Mutex<Vec<PathBuf>>>,
    druid_event_sink: Arc<Mutex<ExtEventSink>>,
    pub dark_theme_enabled: bool,
    blur_enabled: bool,
    filtering_enabled: bool,
}

impl AppState {
    pub fn set_window_id(&mut self, id: WindowId) {
        self.window_id = Some(id);
    }

    pub fn from(dark_theme_enabled: bool, event_sink: ExtEventSink) -> Self {
        Self {
            window_id: None,
            current_image: Arc::new(Mutex::new(ImageState::Empty)),
            command_queue: Arc::new(Mutex::new(vec![])),
            loading_new_image: Arc::new(Mutex::new(false)),
            rotating_image: Arc::new(Mutex::new(false)),
            current_image_index: 0,
            current_image_name: String::new(),
            image_recenter_required: false,
            image_list: Arc::new(Mutex::new(Vec::new())),
            druid_event_sink: Arc::new(Mutex::new(event_sink)),
            dark_theme_enabled,
            blur_enabled: true,
            filtering_enabled: false,
        }
    }

    pub fn has_image(&self) -> bool {
        let image_guard = self.current_image.lock().unwrap();
        match *image_guard {
            ImageState::Loaded(_) => true,
            ImageState::Error(_) => true,
            ImageState::Empty => false,
        }
    }
    pub fn has_image_error(&self) -> bool {
        let image_guard = self.current_image.lock().unwrap();
        match *image_guard {
            ImageState::Loaded(_) => false,
            ImageState::Error(_) => true,
            ImageState::Empty => false,
        }
    }
    pub fn get_image_center_state(&self) -> bool {
        self.image_recenter_required
    }
    pub fn set_image_center_state(&mut self, state: bool) {
        self.image_recenter_required = state;
    }
    pub fn set_image_list(&mut self, index: usize, list: Vec<PathBuf>) {
        self.current_image_index = index;
        self.image_list = Arc::new(Mutex::new(list));
    }

    pub fn get_image_list_size(&self) -> usize {
        let image_list_guard = &self.image_list;
        let image_list = image_list_guard.lock().unwrap();
        image_list.len()
    }

    pub fn startup(&mut self, path: String) {
        let current_time = Instant::now();
        let file_path_result = Path::new(&path).canonicalize();
        if let Ok(file_path) = file_path_result {
            if file_path.is_file() {
                self.set_loading_state(true);
                self.load_image(&file_path, &current_time);
                self.parse_folder(&file_path);
            } else if file_path.is_dir() {
                self.parse_folder(&file_path);

                let first_image: PathBuf;
                {
                    let image_list_guard = &self.image_list;
                    let image_list = image_list_guard.lock().unwrap();
                    first_image = image_list[0].clone();
                }
                self.load_image(&first_image, &current_time);
            }
        } else {
            self.set_loading_state(false);
        }
    }

    fn parse_folder(&mut self, path: &Path) {
        let path_anchor = path.to_path_buf();

        let supported_filetypes = [
            OsStr::new("png"),
            OsStr::new("jpg"),
            OsStr::new("jpeg"),
            OsStr::new("bmp"),
            OsStr::new("ico"),
            OsStr::new("tiff"),
            OsStr::new("webp"),
            OsStr::new("avif"),
            OsStr::new("pnm"),
            OsStr::new("dds"),
            OsStr::new("tga"),
            OsStr::new("exr"),
        ];
        let is_file_supported = |file_extension: &OsStr| {
            for extension in supported_filetypes {
                if file_extension.to_ascii_lowercase() == extension {
                    return true;
                }
            }

            false
        };

        let mut files: Vec<PathBuf> = Vec::new();
        let mut current_index: usize = 0;
        let current_file_name = path_anchor.file_name();
        let current_folder = path_anchor.parent().unwrap();

        for entry in current_folder
            .read_dir()
            .expect("read_dir call failed")
            .flatten()
        {
            if let Some(file_extension) = entry.path().extension() {
                if is_file_supported(file_extension) {
                    files.push(entry.path());
                }
            }
        }

        files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

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

    fn load_image(&mut self, image_path: &Path, request_timestamp: &Instant) {
        let event_sink_mutex_ref = self.druid_event_sink.clone();
        let path_anchor = image_path.to_path_buf();
        let request_timestamp = *request_timestamp;
        thread::spawn(move || {
            let image_result = image::open(&path_anchor);
            let event_sink_mutex = event_sink_mutex_ref.lock().unwrap();
            let event_sink = &*event_sink_mutex;
            if let Ok(image) = image_result {
                let pth = path_anchor.to_str().unwrap().to_string();
                let wrapper = NewImageContainer::from(pth, request_timestamp, image);
                event_sink
                    .submit_command(IMAGE_LOAD_SUCCESS, SingleUse::new(wrapper), Target::Auto)
                    .expect("Failed to send new image loaded command");
            } else {
                event_sink
                    .submit_command(IMAGE_LOAD_FAILURE, path_anchor, Target::Auto)
                    .expect("Failed to submit image loading failure notification command");
            }
        });
    }
    pub fn set_current_image(&mut self, container_wrapper: Option<NewImageContainer>) {
        if let Some(wrapper) = container_wrapper {
            {
                let mut image_guard = self.current_image.lock().unwrap();

                if let ImageState::Empty | ImageState::Error(_) = *image_guard {
                    let new_image = ImageContainer::new(wrapper.image, wrapper.timestamp);
                    *image_guard = ImageState::Loaded(new_image);
                } else if let ImageState::Loaded(current_image) = &*image_guard {
                    if current_image.get_timestamp() < &wrapper.timestamp {
                        let new_image = ImageContainer::new(wrapper.image, wrapper.timestamp);
                        *image_guard = ImageState::Loaded(new_image);
                    }
                }
            }
            let image_name = Path::new(&wrapper.path)
                .file_name()
                .unwrap()
                .to_os_string()
                .into_string()
                .unwrap();
            self.set_current_image_name(image_name);
            self.image_recenter_required = true;
        }
    }
    pub fn image_load_failure(&mut self, image_path: &PathBuf) {
        let image_state_guard = self.get_image_ref();
        let mut image_state = image_state_guard.lock().unwrap();

        let failed_image_placeholder = FailedImageContainer::new();
        *image_state = ImageState::Error(failed_image_placeholder);

        let image_name = Path::new(image_path)
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();
        self.set_current_image_name(image_name);
        self.image_recenter_required = true;
    }
    pub fn get_image_ref(&self) -> Arc<Mutex<ImageState>> {
        self.current_image.clone()
    }
    pub fn get_toolbar_height(&self) -> f64 {
        80.0
    }
    pub fn load_next_image(&mut self, request_timestamp: &Instant) {
        if self.get_image_list_size() > 0 {
            self.set_loading_state(true);
            if self.current_image_index >= self.get_image_list_size() - 1 {
                self.current_image_index = 0;
            } else {
                self.current_image_index += 1;
            }
            let next_image_path: PathBuf;
            {
                let image_list_guard = &self.image_list;
                let image_list = image_list_guard.lock().unwrap();
                next_image_path = image_list[self.current_image_index].clone();
            }
            self.load_image(&next_image_path, request_timestamp);
        }
    }
    pub fn load_prev_image(&mut self, request_timestamp: &Instant) {
        if self.get_image_list_size() > 0 {
            self.set_loading_state(true);
            if self.current_image_index == 0 {
                self.current_image_index = self.get_image_list_size() - 1;
            } else {
                self.current_image_index -= 1;
            }
            let previous_image_path: PathBuf;
            {
                let image_list_guard = &self.image_list;
                let image_list = image_list_guard.lock().unwrap();
                previous_image_path = image_list[self.current_image_index].clone();
            }
            self.load_image(&previous_image_path, request_timestamp);
        }
    }

    pub fn get_image_name(&self) -> String {
        self.current_image_name.clone()
    }

    pub fn set_current_image_name(&mut self, name: String) {
        self.current_image_name = name;
    }

    pub fn set_as_wallpaper(&self) {
        fn set_as_wallpaper_helper(path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
            let path = path
                .into_os_string()
                .into_string()
                .map_err(|_e| "Failed to convert path to string")?;

            thread::spawn(move || {
                let _mode_set_result = wallpaper::set_mode(wallpaper::Mode::Span);
                let _wallpaper_set_result = wallpaper::set_from_path(&path);
            });
            Ok(())
        }

        let image_list_guard = &self.image_list;
        let image_list = image_list_guard.lock().unwrap();
        let _result = set_as_wallpaper_helper(image_list[self.current_image_index].to_path_buf());
    }

    pub fn rotate_in_memory(&mut self, direction: Direction, timestamp: &Instant) {
        if self.get_image_list_size() == 0 {
            return;
        }
        let current_image: DynamicImage;
        {
            let image_state_guard = self.get_image_ref();
            let image_state = image_state_guard.lock().unwrap();
            if let ImageState::Loaded(image) = &*image_state {
                current_image = image.get_image().clone();
            } else {
                return;
            }
        }
        {
            self.set_rotating_state(true);
        }
        {
            self.redraw_widgets();
        }

        let event_sink_mutex_ref = self.druid_event_sink.clone();
        let image_list_guard = &self.image_list;
        let image_list = image_list_guard.lock().unwrap();
        let path_anchor = image_list[self.current_image_index].clone();
        let timestamp = *timestamp;

        thread::spawn(move || {
            let event_sink_mutex = event_sink_mutex_ref.lock().unwrap();
            let event_sink = &*event_sink_mutex;
            let rotated_image = match direction {
                Direction::Left => current_image.rotate270(),
                Direction::Right => current_image.rotate90(),
            };
            let pth = path_anchor.to_str().unwrap().to_string();
            let wrapper = NewImageContainer::from(pth, timestamp, rotated_image);
            event_sink
                .submit_command(
                    IMAGE_ROTATION_COMPLETE,
                    SingleUse::new(wrapper),
                    Target::Auto,
                )
                .expect("Failed to send new image loaded command");
        });
    }

    pub fn copy_image_to_clipboard(&self) {
        let mut clipboard = Application::global().clipboard();
        let image_state_guard = self.get_image_ref();
        let image_state = image_state_guard.lock().unwrap();

        if let ImageState::Loaded(image) = &*image_state {
            let mut clipboard_data_buffer = std::io::Cursor::new(Vec::new());
            image
                .get_image()
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

    pub fn get_rotating_state(&self) -> bool {
        let rotating_state = self.rotating_image.lock().unwrap();
        *rotating_state
    }
    pub fn set_rotating_state(&mut self, new_state: bool) {
        let mut rotating_state = self.rotating_image.lock().unwrap();
        *rotating_state = new_state;
    }

    pub fn show_file_load_dialog(&mut self) {
        if let Some(window_id) = self.window_id {
            let common_image_files = FileSpec::new(
                "Common image files",
                &[
                    "png", "PNG", "jpg", "jpeg", "JPG", "JPEG", "bmp", "BMP", "ico", "ICO", "tiff",
                    "TIFF", "webp", "WEBP", "avif", "AVIF", "pnm", "PNM", "dds", "DDS", "tga",
                    "TGA", "exr", "EXR",
                ],
            );
            let options = FileDialogOptions::new()
                .allowed_types(vec![common_image_files])
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
        let image_state_guard = self.get_image_ref();
        let mut image_state = image_state_guard.lock().unwrap();
        *image_state = ImageState::Empty;
        self.set_image_list(0, Vec::new());
        self.current_image_name = String::new();
        self.image_recenter_required = false;
    }

    pub fn redraw_widgets(&mut self) {
        let event_sink = self.druid_event_sink.lock().unwrap();
        event_sink
            .submit_command(REDRAW_IMAGE, (), Target::Auto)
            .expect("Failed to send redraw command");
    }

    pub fn show_fullscreen_slideshow(&mut self) {
        todo!()
    }

    pub fn delete_image(&mut self) {
        if self.has_image() {
            let path: PathBuf;
            {
                let image_list_guard = &self.image_list;
                let image_list = image_list_guard.lock().unwrap();
                path = image_list[self.current_image_index].to_path_buf();
            }
            let result = trash::delete(path);
            if result.is_ok() {
                if self.get_image_list_size() > 1 {
                    {
                        let deleted_image_index = self.current_image_index;
                        let image_list_guard = &self.image_list;
                        let mut image_list = image_list_guard.lock().unwrap();
                        image_list.remove(deleted_image_index);
                    }
                    self.load_prev_image(&Instant::now());
                } else {
                    self.close_current_image();
                }
            }
        }
    }

    pub fn open_with(&self) {
        if self.has_image() {
            let image_list_guard = &self.image_list;
            let image_list = image_list_guard.lock().unwrap();
            let path = image_list[self.current_image_index].to_path_buf();
            let _result = open_with::open_with(path);
        }
    }

    pub fn open_folder(&self) {
        if self.has_image() {
            let image_list_guard = &self.image_list;
            let image_list = image_list_guard.lock().unwrap();
            let path = image_list[self.current_image_index].to_path_buf();
            let _result = open_with::show_in_folder(path);
        }
    }

    pub fn show_image_properties(&self) {
        if self.has_image() {
            let image_list_guard = &self.image_list;
            let image_list = image_list_guard.lock().unwrap();
            let path = image_list[self.current_image_index].to_path_buf();
            let _result = open_with::show_properties(path);
        }
    }

    pub fn blur_enabled(&self) -> bool {
        self.blur_enabled
    }

    pub fn blur_enable_toggle(&mut self) {
        self.blur_enabled ^= true;
    }

    pub fn image_filtering_enable_toggle(&mut self) {
        self.filtering_enabled ^= true;
    }

    pub fn image_filtering_enabled(&self) -> bool {
        self.filtering_enabled
    }

    pub fn load_prefs(&mut self) {
        let key = "preferences";
        let result = HashMap::<String, bool>::load(&APP_SIG, key);

        if let Ok(preferences) = result {
            preferences
                .get("blur_enabled")
                .map(|b| self.blur_enabled = *b);

            preferences
                .get("image_filtering_enabled")
                .map(|f| self.filtering_enabled = *f);
        }
    }

    pub fn save_prefs(&self) {
        let mut preferences: HashMap<String, bool> = PreferencesMap::new();

        preferences.insert("blur_enabled".into(), self.blur_enabled);
        preferences.insert("image_filtering_enabled".into(), self.filtering_enabled);

        let key = "preferences";
        let _result = preferences.save(&APP_SIG, key);
    }

    pub fn exit(&mut self) {
        self.close_current_image();
        self.save_prefs();
        Application::global().quit()
    }
}
