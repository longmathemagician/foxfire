use crate::data::*;
use crate::image_container::*;
use crate::image_widget::*;
use crate::toolbar_widget::*;
use druid::kurbo::BezPath;
use druid::piet::{Brush, FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{
    Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout,
    WindowDesc,
};
use druid::{Data, WidgetPod};
use std::sync::Arc;

// #[derive(Clone, Data)]
pub struct ContainerWidget {
    image_widget: WidgetPod<AppState, ImageWidget>,
    toolbar: WidgetPod<u32, ToolbarWidget>,
}

impl ContainerWidget {
    pub fn new() -> Self {
        Self {
            image_widget: WidgetPod::new(ImageWidget {}),
            toolbar: WidgetPod::new(ToolbarWidget {}),
        }
    }
}

impl Widget<AppState> for ContainerWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {
        self.image_widget.event(_ctx, _event, _data, _env);
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = _event {
            let mut anchor = _data.get_image_ref();
            let mut image_container = anchor.lock().unwrap();
            let size = image_container.get_size();
            let toolbar_height = _data.get_toolbar_height();
            let image_aspect_ratio = size.width / size.height;
            if (size.width < 800. && size.height < 800.)
                && (size.width > 320. && size.height > 240.)
            {
                let window_size = Size::new(size.width, size.height + toolbar_height);
                _ctx.window().set_size(window_size);
            } else if image_aspect_ratio > 0.5 && image_aspect_ratio < 3. {
                let match_aspect_ratio: Size =
                    Size::new(640., (640. / image_aspect_ratio) + toolbar_height);
                _ctx.window().set_size(match_aspect_ratio);
            }
            let scaled_toolbar_height =
                ((size.height/(_ctx.window().get_size().height-toolbar_height)) * toolbar_height)/2.;
            println!("Displaying image scaled by {}%", scaled_toolbar_height);
            image_container.center_image(scaled_toolbar_height);
        }
        self.image_widget.lifecycle(_ctx, _event, _data, _env);
        self.toolbar.lifecycle(_ctx, _event, &(0 as u32), _env);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        self.image_widget
            .layout(_layout_ctx, &bc.loosen(), _data, _env);
        self.image_widget
            .set_origin(_layout_ctx, _data, _env, Point::new(0.0, 0.0));

        let toolbar_height: f64 = _data.get_toolbar_height();
        let toolbar_layout: BoxConstraints = BoxConstraints::new(
            Size::new(0.0, toolbar_height),
            Size::new(bc.max().width, toolbar_height),
        );
        // toolbar_layout.constrain(Size::new(10., 80.));
        // println!("{:#?}", toolbar_layout);
        self.toolbar
            .layout(_layout_ctx, &toolbar_layout, &(0 as u32), _env);
        self.toolbar.set_origin(
            _layout_ctx,
            &(0 as u32),
            _env,
            Point::new(0.0, bc.max().height - toolbar_height),
        );

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
    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        let fill_color = Color::rgba(0.0, 0.8, 0.25, 1.);
        ctx.fill(rect, &fill_color);
        self.image_widget.paint(ctx, data, env);
        self.toolbar.paint(ctx, &(0 as u32), env);
    }
}
