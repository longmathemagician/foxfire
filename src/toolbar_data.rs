use crate::button_data::*;
use druid::widget::SvgData;
use druid::Data;
use std::sync::{Arc, Mutex};

#[derive(Clone, Data)]
pub struct ToolbarState {
    go_left: bool,
    go_right: bool,
    pub fullscreen_button: ThemedButtonState,
    pub next_button: ThemedButtonState,
    pub prev_button: ThemedButtonState,
}

impl ToolbarState {
    pub fn new() -> Self {
        Self {
            go_left: false,
            go_right: false,
            fullscreen_button: ThemedButtonState::new(),
            next_button: ThemedButtonState::new(),
            prev_button: ThemedButtonState::new(),
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