use crate::types::*;
use druid::Data;

#[derive(Debug, Clone, Data)]
pub struct ZoomEvent {
    delta: f64,           // The distance reported by the scroll event
    position: Vec2D<f64>, // The screen-space point of the scroll event
}

impl ZoomEvent {
    pub fn new(delta: f64, position: Vec2D<f64>) -> Self {
        ZoomEvent { delta, position }
    }
    pub fn get_magnitude(&self) -> f64 {
        self.delta
    }
    pub fn get_position(&self) -> Vec2D<f64> {
        self.position
    }
}

#[derive(Debug, Clone, Data)]
pub struct DragEvent {
    start_pos: Vec2D<f64>,
    delta_pos: Vec2D<f64>,
    finished: bool,
    is_new: bool,
}

impl DragEvent {
    pub fn new(start_pos: Vec2D<f64>, finished: bool) -> Self {
        const ZERO_VECTOR: Vec2D<f64> = Vec2D { x: 0.0, y: 0.0 };
        DragEvent {
            start_pos,
            delta_pos: ZERO_VECTOR,
            finished,
            is_new: true,
        }
    }
    pub fn get_delta(&self) -> Vec2D<f64> {
        self.delta_pos
    }
    pub fn set_delta(&mut self, current_pos: Vec2D<f64>) {
        self.delta_pos = current_pos - self.start_pos;
    }
    pub fn is_finished(&self) -> bool {
        self.finished
    }
    pub fn complete(&mut self) {
        self.finished = true;
    }
    pub fn is_new(&self) -> bool {
        self.is_new
    }
    pub fn mark_seen(&mut self) {
        self.is_new = true
    }
}

#[derive(Debug, Clone, Data)]
pub enum MouseEvent {
    Zoom(ZoomEvent),
    Drag(DragEvent),
}
