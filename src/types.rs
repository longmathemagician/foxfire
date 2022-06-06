use druid::Rect;
use std::fmt;
use std::ops::*;

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
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
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
impl Sub for Position {
    type Output = Position;
    fn sub(self, rhs: Position) -> Self::Output {
        Position::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl Neg for Position {
    type Output = Position;
    fn neg(self) -> Self::Output {
        Position::new(-self.x, -self.y)
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
    zoom_factor: f64,
    drag_position: Position,
}
impl ImageTransformation {
    pub fn new() -> Self {
        ImageTransformation {
            zoom_factor: 1.,
            drag_position: Position::new(0., 0.),
        }
    }
    pub fn get_zoom_factor(&self) -> f64 {
        self.zoom_factor
    }
    pub fn set_zoom_factor(&mut self, zoom_factor: f64) {
        self.zoom_factor = zoom_factor
    }
    pub fn get_drag_position(&self) -> Position {
        self.drag_position
    }
    pub fn set_drag_position(&mut self, drag_position: Position) {
        self.drag_position = drag_position
    }
}
impl Clone for ImageTransformation {
    fn clone(&self) -> Self {
        ImageTransformation {
            zoom_factor: self.zoom_factor,
            drag_position: self.drag_position,
        }
    }
}
