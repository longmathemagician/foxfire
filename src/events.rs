use crate::types::*;

#[derive(Debug)]
pub struct ClickEvent {
    position: Position,
}
impl ClickEvent {
    pub fn new(position: Position) -> Self { ClickEvent { position } }
    pub fn get_position(&self) -> Position { self.position }
}

#[derive(Debug)]
pub struct ZoomEvent {
    delta: f64, // The distance reported by the scroll event
    position: Position, // The screen-space point of the scroll event
}
impl ZoomEvent {
    pub fn new(delta: f64, position: Position) -> Self {
        ZoomEvent { delta, position }
    }
    pub fn get_magnitude(&self) -> f64 { self.delta }
    pub fn get_position(&self) -> Position { self.position }
    pub fn set_position(&mut self, position: Position) { self.position = position }
}
#[derive(Debug)]
pub struct DragEvent {
    start_pos: Position,
    delta_pos: Position,
    finished: bool,
}
impl DragEvent {
    pub fn new(start_pos: Position, finished: bool,) -> Self {
        let delta_pos = Position::new(0., 0.);
        DragEvent {
            start_pos,
            delta_pos,
            finished,
        }
    }
    pub fn get_delta(&self) -> Position {
        self.delta_pos
    }
    pub fn set_delta(&mut self, current_pos: Position) {
        self.delta_pos.set(
            current_pos.x() - self.start_pos.x(),
            current_pos.y() - self.start_pos.y(),
        );
    }
    pub fn is_finished(&self) -> bool {
        self.finished
    }
    pub fn complete(&mut self) {
        self.finished = true;
    }
}
#[derive(Debug)]
pub enum MouseEvent {
    Zoom(ZoomEvent),
    Drag(DragEvent),
    Click(ClickEvent),
}