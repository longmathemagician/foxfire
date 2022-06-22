use druid::Data;

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
    pub fn set_pressed(&mut self, state: bool) {
        self.pressed = state;
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
