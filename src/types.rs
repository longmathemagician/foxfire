use std::ops::*;
use std::fmt;
use druid::Rect;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}
impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }
    pub fn set(&mut self, x: f64, y: f64) {
        self.x = x;
        self.y = y;
    }
    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}
impl Add for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Position {
        Position::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Position) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub struct ImageTransformation {
    pub zoom_factor: f64,
    pub viewport: Option<Rect>,
    zoom_target: Position,
    drag_position: Position,
}
impl ImageTransformation {
    pub fn new() -> Self {
        ImageTransformation {
            zoom_factor: 0.,
            viewport: None,
            zoom_target: Position::new(0., 0.),
            drag_position: Position::new(0., 0.),
        }
    }
    pub fn set_zoom() {

    }
    pub fn get_zoom_factor(&self) -> f64 { self.zoom_factor }
    pub fn set_zoom_factor(&mut self, zoom_factor: f64) { self.zoom_factor = zoom_factor }
    pub fn get_zoom_target(&self) -> Position { self.zoom_target }
    pub fn set_zoom_target(&mut self, zoom_target: Position) { self.zoom_target = zoom_target }
    pub fn get_drag_position(&self) -> Position { self.drag_position }
    pub fn set_drag_position(&mut self, drag_position: Position) { self.drag_position = drag_position }
}