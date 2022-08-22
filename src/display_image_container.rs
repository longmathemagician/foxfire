use druid::piet::PietImage;
use druid::widget::prelude::*;
use image::DynamicImage;

use crate::events::*;

// #[derive(Clone, Data)]
pub struct DisplayImageContainer {
    image_data: Option<DynamicImage>,
    image_size: Size,
    image_cache: Option<PietImage>,
    pub event_queue: Option<MouseEvent>,
}

impl DisplayImageContainer {
    pub fn new() -> Self {
        Self {
            image_data: None,
            image_size: Size::new(0.0, 0.0),
            image_cache: None,
            event_queue: None,
        }
    }
    pub fn get_size(&self) -> Size {
        self.image_size
    }
    pub fn get_image(&self) -> Option<&DynamicImage> {
        self.image_data.as_ref()
    }
    pub fn has_cache(&self) -> bool {
        matches!(self.image_cache, Some(_))
    }
    pub fn set_cache(&mut self, cached_image: PietImage) {
        self.image_cache = Some(cached_image);
    }
    pub fn get_cache(&self) -> &PietImage {
        (self.image_cache.as_ref()).unwrap()
    }
    pub fn set_image(&mut self, new_image: DynamicImage) {
        let size = Size::new(new_image.width() as f64, new_image.height() as f64);
        self.image_size = size;
        self.image_data = Some(new_image);
        self.image_cache = None;
    }

    pub fn clear_image(&mut self) {
        self.image_data = None;
        self.image_size = Size::new(0.0, 0.0);
        self.image_cache = None;
        self.event_queue = None;
    }
}
