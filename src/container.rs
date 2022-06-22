use druid::widget::prelude::*;
use druid::WidgetPod;
use druid::{Color, KbKey, Point};

use crate::app_state::*;
use crate::image_widget::*;
use crate::toolbar_data::*;
use crate::toolbar_widget::*;
use crate::types::Direction;

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
        if let Event::KeyDown(k) = _event {
            if k.key == KbKey::ArrowRight {
                _data.load_next_image();
            } else if k.key == KbKey::ArrowLeft {
                _data.load_prev_image();
            }
        } else if let Event::MouseDown(e)
        | Event::MouseUp(e)
        | Event::MouseMove(e)
        | Event::Wheel(e) = _event
        {
            if e.window_pos.y < _ctx.size().height - _data.get_toolbar_height() {
                _ctx.set_focus(self.image_widget.id());
                self.image_widget.event(_ctx, _event, _data, _env);
            } else {
                _ctx.set_focus(self.toolbar.id());
                let anchor = _data.get_toolbar_state();
                let mut toolbar_state = anchor.lock().unwrap();
                self.toolbar.event(_ctx, _event, &mut toolbar_state, _env);
            }
        } else {
            self.image_widget.event(_ctx, _event, _data, _env);
        }

        let anchor = _data.get_toolbar_state();
        let mut tb_state = anchor.lock().unwrap();
        if tb_state.get_right() {
            _data.load_next_image();
            tb_state.set_right(false);
        } else if tb_state.get_left() {
            _data.load_prev_image();
            tb_state.set_left(false);
        } else if tb_state.get_recenter() {
            _data.image_centered = true;
            _data.recenter_on_next_paint();
            tb_state.set_recenter(false);
            _ctx.request_paint();
        } else if tb_state.get_rotate_left() {
            _data.rotate_in_memory(Direction::Left);
            tb_state.set_rotate_left(false);
            _ctx.request_paint();
        } else if tb_state.get_rotate_right() {
            _data.rotate_in_memory(Direction::Right);
            tb_state.set_rotate_right(false);
            _ctx.request_paint();
        }

        let resizing = match _event {
            Event::WindowSize(_) => true,
            _ => false,
        };
        if (_data.get_image_freshness() | (resizing && _data.image_centered))
            && _ctx.size().width > 0.
        {
            _data.set_image_freshness(false);
            let mut new_title = "Foxfire - ".to_string();
            new_title.push_str(&*_data.get_image_name());
            _ctx.window().set_title(&new_title);
            let anchor = _data.get_image_ref();
            let mut image_container = anchor.lock().unwrap();
            let size = image_container.get_size();
            let toolbar_height = _data.get_toolbar_height();

            image_container.center_image(_ctx.size(), toolbar_height);
            _data.image_centered = true;
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
        self.image_widget.lifecycle(_ctx, _event, _data, _env);

        let anchor = _data.get_toolbar_state();
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

        let anchor = _data.get_toolbar_state();
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

        let anchor = data.get_toolbar_state();
        let toolbar_state = anchor.lock().unwrap();

        self.toolbar.paint(ctx, &toolbar_state, env);
    }
}
