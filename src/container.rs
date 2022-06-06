use crate::app_state::*;
use crate::image_container::*;
use crate::image_widget::*;
use crate::toolbar_data::*;
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
    toolbar: WidgetPod<ToolbarState, ToolbarWidget>,
}

impl ContainerWidget {
    pub fn new() -> Self {
        Self {
            image_widget: WidgetPod::new(ImageWidget {}),
            toolbar: WidgetPod::new(ToolbarWidget::new()),
        }
    }
}

impl Widget<AppState> for ContainerWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {
        if let Event::MouseDown(e) | Event::MouseUp(e) | Event::MouseMove(e) | Event::Wheel(e) =
            _event
        {
            if e.window_pos.y < _ctx.window().get_size().height - _data.get_toolbar_height() {
                self.image_widget.event(_ctx, _event, _data, _env);
            } else if e.window_pos.y == _ctx.window().get_size().height - _data.get_toolbar_height()
            {
                self.image_widget.event(_ctx, _event, _data, _env)
            } else {
                let mut anchor = _data.get_toolbar_state();
                let mut toolbar_state = anchor.lock().unwrap();
                self.toolbar.event(_ctx, _event, &mut toolbar_state, _env);
            }
        } else {
            self.image_widget.event(_ctx, _event, _data, _env);
        }

        let mut anchor = _data.get_toolbar_state();
        let mut tb_state = anchor.lock().unwrap();
        if tb_state.get_right() {
            _data.load_next_image();
            tb_state.set_right(false);
        } else if tb_state.get_left() {
            _data.load_prev_image();
            tb_state.set_left(false);
        }
        // _ctx.request_paint();
        if _data.get_image_freshness() {
            _data.set_image_freshness(false);
            let mut new_title = "🦜 Photo Viewer - ".to_string();
            new_title.push_str(&*_data.get_image_name());
            _ctx.window().set_title(&new_title);
            let mut anchor = _data.get_image_ref();
            let mut image_container = anchor.lock().unwrap();
            let size = image_container.get_size();
            let toolbar_height = _data.get_toolbar_height();
            let image_aspect_ratio = size.width / size.height;

            let scaled_toolbar_height = ((size.height
                / (_ctx.window().get_size().height - toolbar_height))
                * toolbar_height)
                / 2.;
            println!("Displaying image scaled by {}%", scaled_toolbar_height);
            image_container.center_image(
                _ctx.window().get_size(),
                toolbar_height,
                scaled_toolbar_height,
            );
            _ctx.request_paint();
        }
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
            let scaled_toolbar_height = ((size.height
                / (_ctx.window().get_size().height - toolbar_height))
                * toolbar_height)
                / 2.;
            println!("Displaying image scaled by {}%", scaled_toolbar_height);
            image_container.center_image(
                _ctx.window().get_size(),
                toolbar_height,
                scaled_toolbar_height,
            );
        }

        self.image_widget.lifecycle(_ctx, _event, _data, _env);

        let mut anchor = _data.get_toolbar_state();
        let toolbar_state = anchor.lock().unwrap();
        self.toolbar.lifecycle(_ctx, _event, &toolbar_state, _env);
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

        let mut anchor = _data.get_toolbar_state();
        let toolbar_state = anchor.lock().unwrap();

        self.toolbar
            .layout(_layout_ctx, &toolbar_layout, &toolbar_state, _env);
        self.toolbar.set_origin(
            _layout_ctx,
            &toolbar_state,
            _env,
            Point::new(0.0, bc.max().height - toolbar_height),
        );

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

        let mut anchor = data.get_toolbar_state();
        let toolbar_state = anchor.lock().unwrap();

        self.toolbar.paint(ctx, &toolbar_state, env);
    }
}
