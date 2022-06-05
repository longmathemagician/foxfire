use crate::container::*;
use crate::files::*;
use crate::image_container::*;
use crate::image_widget::*;
use crate::toolbar_data::*;
use crate::toolbar_widget::*;
use image::*;
use image::DynamicImage;
use druid::widget::Button;
use druid::Color;
use druid::{Data, WidgetPod};
use std::borrow::{Borrow, BorrowMut};
use std::sync::{Arc, Mutex};

#[derive(Clone, Data)]
pub struct AppState {
    current_image: Arc<Mutex<ImageContainer>>,
    image_loader: Arc<Mutex<AsyncImageLoader>>,
    toolbar_state: Arc<Mutex<ToolbarState>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_image: Arc::new(Mutex::new(ImageContainer::new())),
            image_loader: Arc::new(Mutex::new(AsyncImageLoader::new())),
            toolbar_state: Arc::new(Mutex::new(ToolbarState::new()))
        }
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
}
