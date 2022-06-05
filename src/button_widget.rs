use std::sync::{Arc, Mutex};
use druid::Data;
use druid::widget::prelude::*;
use druid::Color;
use druid::piet::{ImageFormat, InterpolationMode, PietImage};
use crate::toolbar_data::*;
use crate::button_data::*;

// #[derive(Clone, Data)]
pub struct ThemedButton {
    image_cached: Option<PietImage>,
    image_hot_cached: Option<PietImage>,
}
impl ThemedButton {
    pub fn new() -> Self {
        Self {
            image_cached: None,
            image_hot_cached: None,
        }
    }
}
impl Widget<ThemedButtonState> for ThemedButton {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut ThemedButtonState, _env: &Env) {

    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &ThemedButtonState,
        _env: &Env,
    ) {

    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &ThemedButtonState, _data: &ThemedButtonState, _env: &Env) {

    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &ThemedButtonState,
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max();
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size);
        }
        Size::new(450.0, 64.0)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &ThemedButtonState, env: &Env) {
        let container_size = ctx.size();
        let container_rect = container_size.to_rect();
        // ctx.fill(container_rect, &Color::YELLOW);
        if data.is_hot() {

        } else {
            if self.image_cached.is_none() {
                let image = data.get_image();
                let image_result = ctx.make_image(
                    image.width() as usize,
                    image.height() as usize,
                    image.as_bytes(),
                    ImageFormat::RgbaPremul,
                );
                self.image_cached = Some(image_result.unwrap())
            }
            ctx.draw_image(
                self.image_cached.as_ref().unwrap(),
                container_rect,
                InterpolationMode::NearestNeighbor,
            );
        }
        // let mut anchor = data.get_image_ref();
        // let mut image_container = anchor.lock().unwrap();
        // if !image_container.has_cache() {
        //     let cached_image_size = image_container.get_size();
        //     let image_result = ctx.make_image(
        //         cached_image_size.width as usize,
        //         cached_image_size.height as usize,
        //         image_container.get_image().as_bytes(),
        //         ImageFormat::Rgb,
        //     );
        //     image_container.set_cache(image_result.unwrap());
        // }
        // let image_size = image_container.get_size();
        //
        // ctx.draw_image_area(
        //     image_container.get_cache(),
        //     output_viewport,
        //     container_viewport,
        //     InterpolationMode::NearestNeighbor,
        // );
    }
}