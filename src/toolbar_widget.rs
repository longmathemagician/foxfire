use crate::data::*;
use crate::image_container::*;
use druid::kurbo::BezPath;
use druid::piet::{Brush, FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{
    Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout,
    WindowDesc,
};
use std::sync::Arc;

#[derive(Clone, Data)]
pub struct ToolbarWidget;

impl Widget<u32> for ToolbarWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut u32, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &u32, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &u32, _data: &u32, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &u32,
        _env: &Env,
    ) -> Size {
        // BoxConstraints are passed by the parent widget.
        // This method can return any Size within those constraints:
        // bc.constrain(my_size)
        //
        // To check if a dimension is infinite or not (e.g. scrolling):
        // bc.is_width_bounded() / bc.is_height_bounded()
        //
        // bx.max() returns the maximum size of the widget. Be careful
        // using this, since always make sure the widget is bounded.
        // If bx.max() is used in a scrolling widget things will probably
        // not work correctly.
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    // It goes event -> update -> layout -> paint, and each method can influence the next.
    fn paint(&mut self, ctx: &mut PaintCtx, data: &u32, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        let fill_color = Color::rgba(1., 1., 1., 0.8);
        ctx.fill(rect, &fill_color);
    }
}
