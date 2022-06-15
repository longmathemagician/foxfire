use std::fmt;
use std::ops::*;
use std::ops::{Add, Mul};

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

#[derive(Debug, Copy, Clone)]
pub struct ImageTransformation {
    pub affine_matrix: Matrix2x2<f64>,
    pub image_space_offset: Vec2D<f64>,
    pub screen_space_offset: Vec2D<f64>,
}

impl ImageTransformation {
    pub fn new() -> Self {
        ImageTransformation {
            affine_matrix: Matrix2x2::new(),
            image_space_offset: Vec2D::new(),
            screen_space_offset: Vec2D::new(),
        }
    }
    pub fn set_scale(&mut self, scale_factor: f64) {
        self.affine_matrix.scale(scale_factor);
    }
    pub fn set_screen_space_offset(&mut self, new_offset: Vec2D::<f64>) {
        self.screen_space_offset = new_offset;
    }
}

pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

impl<T: From<f32>> Vec2D<T> {
    pub fn new() -> Self {
        Self {
            x: T::from(0.0),
            y: T::from(0.0),
        }
    }
    pub fn from(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T> Add<Vec2D<T>> for Vec2D<T>
    where
        T: From<f32> + std::ops::Add<Output=T> + std::ops::Add<Output=T> + Copy,
{
    type Output = Vec2D<T>;

    fn add(self, rhs: Vec2D<T>) -> Self::Output {
        Vec2D::from(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub<Vec2D<T>> for Vec2D<T>
    where
        T: From<f32> + std::ops::Sub<Output=T> + std::ops::Sub<Output=T> + Copy,
{
    type Output = Vec2D<T>;

    fn sub(self, rhs: Vec2D<T>) -> Self::Output {
        Vec2D::from(self.x - rhs.x, self.y - rhs.y)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Matrix2x2<T> {
    pub a: T,
    pub b: T,
    pub c: T,
    pub d: T,
}

impl<T: From<f32> + std::ops::Mul<Output=T> + std::ops::Div<Output=T> + std::cmp::PartialOrd + std::ops::Add<Output=T> + std::ops::MulAssign + std::fmt::Debug + Copy + std::ops::Sub<Output=T>> Matrix2x2<T> {
    pub fn new() -> Self {
        Self {
            a: T::from(1.0),
            b: T::from(0.0),
            c: T::from(0.0),
            d: T::from(1.0),
        }
    }
    pub fn scale(&mut self, scale_factor: T) {
        let new_scale: T = self.a * scale_factor;
        if new_scale <= T::from(100.0) && new_scale >= T::from(0.1) {
            self.a = new_scale;
            self.d = new_scale;
        }
    }
    pub fn inverse(&self) -> Self {
        let det = T::from(1.0) / (self.a * self.d - self.b * self.c);
        Self {
            a: det * self.d,
            b: det * (T::from(0.0) - self.b),
            c: det * (T::from(0.0) - self.c),
            d: det * (self.a),
        }
    }
}

impl<T> Mul<Vec2D<T>> for Matrix2x2<T>
    where
        T: From<f32> + std::ops::Mul<Output=T> + std::ops::Add<Output=T> + Copy,
{
    type Output = Vec2D<T>;

    fn mul(self, rhs: Vec2D<T>) -> Self::Output {
        Vec2D::from(self.a * rhs.x + self.b * rhs.y, self.c * rhs.x + self.d * rhs.y)
    }
}