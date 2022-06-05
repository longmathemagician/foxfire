use druid::Data;
use image::*;
use image::DynamicImage;
use std::sync::{Arc, Mutex};
use druid::piet::PietImage;

#[derive(Clone, Data)]
pub struct ThemedButtonState {
	pressed: bool,
	hot: bool,
    pub image: Arc<DynamicImage>,
	pub image_hot: Arc<DynamicImage>,
}

impl ThemedButtonState {
    pub fn new(image: Arc<DynamicImage>, image_hot: Arc<DynamicImage>) -> Self {
        Self {
			pressed: false,
			hot: false,
			image,
			image_hot,
        }
    }
	pub fn is_pressed(&self) -> bool {
		self.pressed
	}
	pub fn is_hot(&self) -> bool {
		self.hot
	}
	pub fn set_pressed(&mut self, state: bool) {
		self.pressed = state;
	}
	pub fn set_hot(&mut self, state: bool) {
		self.hot = state;
	}
	pub fn get_image(&self) -> Arc<DynamicImage> {
		self.image.clone()
	}
}
