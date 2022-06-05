use druid::piet::PietImage;
use druid::widget::SvgData;
use druid::Data;
use std::sync::{Arc, Mutex};

#[derive(Clone, Data)]
pub struct ThemedButtonState {
    event: bool,
    pressed: bool,
    hot: bool,
}

impl ThemedButtonState {
    pub fn new() -> Self {
        Self {
            event: false,
            pressed: false,
            hot: false,
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
    pub fn fire_event(&mut self) {
        self.event = true;
    }
    pub fn clear_event(&mut self) {
        self.event = false;
    }
    pub fn has_event(&self) -> bool {
        self.event
    }
}
