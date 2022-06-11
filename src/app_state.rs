use crate::container::*;
use crate::files::*;
use crate::image_container::*;
use crate::image_widget::*;
use crate::toolbar_data::*;
use crate::toolbar_widget::*;
use druid::widget::Button;
use druid::Color;
use druid::{Data, WidgetPod};
use image::DynamicImage;
use image::*;
use std::borrow::{Borrow, BorrowMut};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use wallpaper;

#[derive(Clone, Data)]
pub struct AppState {
    current_image: Arc<Mutex<ImageContainer>>,
    current_image_index: usize,
    current_image_name: String,
    image_recenter_required: bool,
    image_list: Arc<Vec<PathBuf>>,
    image_loader: Arc<Mutex<AsyncImageLoader>>,
    toolbar_state: Arc<Mutex<ToolbarState>>,
    pub dark_theme_enabled: bool,
}

impl AppState {
    pub fn new(dark_theme_enabled: bool) -> Self {
        Self {
            current_image: Arc::new(Mutex::new(ImageContainer::new())),
            current_image_index: 0,
            current_image_name: String::new(),
            image_recenter_required: true,
            image_list: Arc::new(Vec::new()),
            image_loader: Arc::new(Mutex::new(AsyncImageLoader::new())),
            toolbar_state: Arc::new(Mutex::new(ToolbarState::new(dark_theme_enabled))),
            dark_theme_enabled,
        }
    }
    pub fn get_image_freshness(&self) -> bool {
        self.image_recenter_required
    }
    pub fn set_image_freshness(&mut self, state: bool) {
        self.image_recenter_required = state;
    }
    pub fn set_image_list(&mut self, index: usize, list: Vec<PathBuf>) {
        self.current_image_index = index;
        self.image_list = Arc::new(list);
    }
    pub fn set_current_image(&mut self) {
        let mut tmp = self.image_loader.lock().unwrap();
        // Receive the image from the thread
        if tmp.has_receiver() {
            if let Some(mut received_image_handle) = tmp.take_image_receiver() {
                let potential_image = received_image_handle.recv();
                if let Ok(new_image) = potential_image {
                    let mut current_image = self.current_image.lock().unwrap();
                    current_image.set_image(new_image);
                }
            }
        } else if tmp.has_image() {
            if let Some(image) = tmp.take_image() {
                let mut current_image = self.current_image.lock().unwrap();
                current_image.set_image(image);
            }
        }
    }
    pub fn set_image_handler(&mut self, image_receiver: Arc<Mutex<AsyncImageLoader>>) {
        self.image_loader = image_receiver;
    }
    pub fn get_image_ref(&self) -> Arc<Mutex<ImageContainer>> {
        self.current_image.clone()
    }
    pub fn get_toolbar_height(&self) -> f64 {
        80.0
    }
    pub fn get_toolbar_state(&self) -> Arc<Mutex<ToolbarState>> {
        self.toolbar_state.clone()
    }
    pub fn load_next_image(&mut self) {
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
        println!("Current image name: {}", self.current_image_name);
        let mut image_receiver = AsyncImageLoader::new_from_string(
            self.image_list[self.current_image_index]
                .clone()
                .to_str()
                .unwrap(),
        );
        image_receiver.load_image();
        if let Some(mut received_image_handle) = image_receiver.take_image_receiver() {
            let potential_image = received_image_handle.recv();
            if let Ok(new_image) = potential_image {
                let mut current_image = self.current_image.lock().unwrap();
                current_image.set_image(new_image);
                self.image_recenter_required = true;
            }
        }
    }
    pub fn load_prev_image(&mut self) {
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
        println!("Current image name: {}", self.current_image_name);
        let mut image_receiver = AsyncImageLoader::new_from_string(
            self.image_list[self.current_image_index]
                .clone()
                .to_str()
                .unwrap(),
        );
        image_receiver.load_image();
        if let Some(mut received_image_handle) = image_receiver.take_image_receiver() {
            let potential_image = received_image_handle.recv();
            if let Ok(new_image) = potential_image {
                let mut current_image = self.current_image.lock().unwrap();
                current_image.set_image(new_image);
                self.image_recenter_required = true;
            }
        }
    }
    pub fn get_image_name(&self) -> String {
        self.current_image_name.clone()
    }
    pub fn set_current_image_name(&mut self, name: String) {
        self.current_image_name = name;
    }
    pub fn recenter_on_next_paint(&mut self) {
        self.image_recenter_required = true;
    }
    pub fn set_as_wallpaper(&self) {
        wallpaper::set_mode(wallpaper::Mode::Span);
        wallpaper::set_from_path(&self.image_list[self.current_image_index]
            .clone()
            .to_str()
            .unwrap()
            .to_string()
        );
    }
}
