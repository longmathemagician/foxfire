use std::time::Instant;

use druid::piet::PietImage;
use druid::widget::prelude::*;
use image::DynamicImage;

use crate::events::*;

#[derive(Clone, Data)]
pub enum ImageState {
    Empty,
    Loaded(ImageContainer),
    Error(FailedImageContainer),
}

#[derive(Clone, Data)]
pub struct FailedImageContainer {}

impl FailedImageContainer {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Clone, Data)]
pub struct ImageContainer {
    load_request_timestamp: Instant,
    #[data(ignore)]
    image_data: DynamicImage,
    image_size: Size,
    #[data(ignore)]
    image_cache: Option<PietImage>,
    #[data(ignore)]
    pub event_queue: Option<MouseEvent>,
}

impl ImageContainer {
    pub fn new(image_data: DynamicImage, load_request_timestamp: Instant) -> Self {
        let image_size = Size::new(image_data.width() as f64, image_data.height() as f64);
        Self {
            load_request_timestamp,
            image_data,
            image_size,
            image_cache: None,
            event_queue: None,
        }
    }
    pub fn get_timestamp(&self) -> &Instant {
        &self.load_request_timestamp
    }
    pub fn get_size(&self) -> Size {
        self.image_size
    }
    pub fn get_image(&self) -> &DynamicImage {
        &self.image_data
    }
    pub fn has_cache(&self) -> bool {
        matches!(self.image_cache, Some(_))
    }
    pub fn set_cache(&mut self, cached_image: PietImage) {
        self.image_cache = Some(cached_image);
    }
    pub fn get_cache(&self) -> Option<&PietImage> {
        self.image_cache.as_ref()
    }
}
