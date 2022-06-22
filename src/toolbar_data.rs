use crate::button_data::*;
use druid::Data;

#[derive(Clone, Data)]
pub struct ToolbarState {
    go_left: bool,
    go_right: bool,
    recenter: bool,
    rotate_left_event: bool,
    rotate_right_event: bool,
    pub fullscreen_button: ThemedButtonState,
    pub next_button: ThemedButtonState,
    pub prev_button: ThemedButtonState,
    pub rotate_right_button: ThemedButtonState,
    pub rotate_left_button: ThemedButtonState,
    pub delete_button: ThemedButtonState,
    pub recenter_button: ThemedButtonState,
    pub zoom_button: ThemedButtonState,
    pub dark_theme_enabled: bool,
}

impl ToolbarState {
    pub fn new(dark_theme_enabled: bool) -> Self {
        Self {
            go_left: false,
            go_right: false,
            recenter: false,
            rotate_left_event: false,
            rotate_right_event: false,
            fullscreen_button: ThemedButtonState::new(),
            next_button: ThemedButtonState::new(),
            prev_button: ThemedButtonState::new(),
            rotate_right_button: ThemedButtonState::new(),
            rotate_left_button: ThemedButtonState::new(),
            delete_button: ThemedButtonState::new(),
            recenter_button: ThemedButtonState::new(),
            zoom_button: ThemedButtonState::new(),
            dark_theme_enabled,
        }
    }

    pub fn get_left(&self) -> bool {
        self.go_left
    }
    pub fn set_left(&mut self, state: bool) {
        self.go_left = state;
    }

    pub fn get_right(&self) -> bool {
        self.go_right
    }
    pub fn set_right(&mut self, state: bool) {
        self.go_right = state;
    }

    pub fn get_recenter(&self) -> bool {
        self.recenter
    }
    pub fn set_recenter(&mut self, state: bool) {
        self.recenter = state;
    }

    pub fn get_rotate_left(&self) -> bool {
        self.rotate_left_event
    }
    pub fn set_rotate_left(&mut self, state: bool) {
        self.rotate_left_event = state;
    }

    pub fn get_rotate_right(&self) -> bool {
        self.rotate_right_event
    }
    pub fn set_rotate_right(&mut self, state: bool) {
        self.rotate_right_event = state;
    }
}
