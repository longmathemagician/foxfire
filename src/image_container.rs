use druid::piet::PietImage;
use druid::widget::prelude::*;
use image::DynamicImage;

use crate::events::*;
use crate::types::*;

// #[derive(Clone, Data)]
pub struct ImageContainer {
    image_data: Option<DynamicImage>,
    image_size: Size,
    image_cache: Option<PietImage>,
    pub event_queue: Option<MouseEvent>,
    pub transform: ImageTransformation,
}

impl ImageContainer {
    pub fn new() -> Self {
        Self {
            image_data: None,
            image_size: Size::new(0.0, 0.0),
            image_cache: None,
            event_queue: None,
            transform: ImageTransformation::new(),
        }
    }
    pub fn get_size(&self) -> Size {
        self.image_size
    }
    pub fn get_image(&self) -> &DynamicImage {
        self.image_data.as_ref().unwrap()
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
    pub fn peek_event_queue(&self) -> Option<&MouseEvent> {
        self.event_queue.as_ref()
    }
    pub fn push_event_queue(&mut self, event: MouseEvent) {
        self.event_queue = Some(event);
    }
    pub fn pop_event_queue(&mut self) -> Option<MouseEvent> {
        self.event_queue.take()
    }
    pub fn set_image(&mut self, new_image: DynamicImage) {
        let size = Size::new(new_image.width() as f64, new_image.height() as f64);
        self.image_size = size;
        self.image_data = Some(new_image);
        self.image_cache = None;
    }
    pub fn center_image(&mut self, container: Size, unscaled_toolbar_offset: f64) {
        if self.image_data.is_some() {
            self.transform = ImageTransformation::new();
            let image = self.image_size;
            let image_aspect_ratio = image.width / image.height;
            let container_aspect_ratio =
                container.width / (container.height - unscaled_toolbar_offset);
            self.transform = ImageTransformation::new();

            let scale_factor: f64;
            let centering_vector: Vec2D<f64>;

            if image_aspect_ratio > container_aspect_ratio {
                // the image is wider than the container, so match the widths to fill
                scale_factor = container.width / image.width;
                centering_vector = Vec2D::from(
                    0.,
                    (container.height - unscaled_toolbar_offset) / 2.
                        - (image.height * scale_factor) / 2.,
                );
            } else {
                // the image is wider than the container, so fit the heights
                scale_factor = (container.height - unscaled_toolbar_offset) / image.height;
                centering_vector =
                    Vec2D::from(container.width / 2. - (image.width * scale_factor) / 2., 0.);
            }

            self.transform.set_screen_space_offset(centering_vector);
            self.transform.set_scale(scale_factor);
        }
    }
}
