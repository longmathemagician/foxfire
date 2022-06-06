use crate::events::*;
use crate::files::*;
use crate::types::*;
use druid::piet::{Piet, PietImage};
use druid::widget::prelude::*;
use image::DynamicImage;

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
        match self.image_cache {
            Some(_) => true,
            _ => false,
        }
    }
    pub fn set_cache(&mut self, cached_image: PietImage) {
        self.image_cache = Some(cached_image);
    }
    pub fn get_cache(&self) -> &PietImage {
        &(self.image_cache.as_ref()).unwrap()
    }
    pub fn peek_event_queue(&self) -> Option<&MouseEvent> {
        match &self.event_queue {
            Some(event) => Some(&event),
            _ => None,
        }
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
    pub fn center_image(
        &mut self,
        container: Size,
        unscaled_toolbar_offset: f64,
        scaled_toolbar_offset: f64,
    ) {
        self.transform = ImageTransformation::new();
        let image = self.image_size;
        let image_aspect_ratio = image.width / image.height;
        let container_aspect_ratio = container.width / (container.height - unscaled_toolbar_offset);
        self.transform = ImageTransformation::new();
        if self.image_data.is_some() {
            let centered_position: Position =
                Position::new(image.width / 2., image.height / 2. + scaled_toolbar_offset);
            self.transform.set_drag_position(centered_position);
        }
    }
}
