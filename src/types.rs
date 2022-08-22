use std::ops::*;

use image::DynamicImage;

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
    pub fn set_screen_space_offset(&mut self, new_offset: Vec2D<f64>) {
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
        Self { x, y }
    }
}

impl<T> Add<Vec2D<T>> for Vec2D<T>
where
    T: From<f32> + Add<Output = T> + Add<Output = T> + Copy,
{
    type Output = Vec2D<T>;

    fn add(self, rhs: Vec2D<T>) -> Self::Output {
        Vec2D::from(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> Sub<Vec2D<T>> for Vec2D<T>
where
    T: From<f32> + Sub<Output = T> + Sub<Output = T> + Copy,
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

impl<
        T: From<f32>
            + Mul<Output = T>
            + Div<Output = T>
            + PartialOrd
            + Add<Output = T>
            + MulAssign
            + Copy
            + Sub<Output = T>,
    > Matrix2x2<T>
{
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
        if new_scale <= T::from(100.0) && new_scale >= T::from(0.01) {
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
    T: From<f32> + Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Vec2D<T>;

    fn mul(self, rhs: Vec2D<T>) -> Self::Output {
        Vec2D::from(
            self.a * rhs.x + self.b * rhs.y,
            self.c * rhs.x + self.d * rhs.y,
        )
    }
}

pub struct NewImageContainer {
    pub path: String,
    pub image: DynamicImage,
}

impl NewImageContainer {
    pub fn from_string_and_dynamicimage(path: String, image: DynamicImage) -> Self {
        Self { path, image }
    }
}
