use druid::Data;
use image::*;
use image::DynamicImage;
use std::sync::{Arc, Mutex};
use crate::button_data::*;

#[derive(Clone, Data)]
pub struct ToolbarState {
    go_left: bool,
    go_right: bool,
	pub fullscreen_button: ThemedButtonState,
}

impl ToolbarState {
    pub fn new() -> Self {
		let fsb_normal = image::load_from_memory(include_bytes!("../resources/buttons/fullscreen.png")).unwrap();
		let fsb_hot = image::load_from_memory(include_bytes!("../resources/buttons/fullscreen_hot.png")).unwrap();
        Self {
			go_left: false,
			go_right: false,
			fullscreen_button: ThemedButtonState::new(Arc::new(fsb_normal), Arc::new(fsb_hot)),
        }
    }
    pub fn get_left(&self) -> bool {
		self.go_left
	}
	pub fn get_right(&self) -> bool {
		self.go_right
	}
	pub fn set_left(&mut self, state: bool) {
		self.go_left = state;
	}
	pub fn set_right(&mut self, state: bool) {
		self.go_right = state;
	}
}
