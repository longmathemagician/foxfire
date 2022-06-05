use crate::app_state::*;
use crate::button_data::*;
use crate::button_widget::*;
use crate::image_container::*;
use crate::toolbar_data::*;
use druid::kurbo::BezPath;
use druid::piet::{Brush, FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::widget::SvgData;
use druid::{
    Affine, AppLauncher, Color, FontDescriptor, LocalizedString, Point, Rect, TextLayout,
    WindowDesc,
};
use druid::{Data, WidgetPod};
use std::sync::Arc;

// #[derive(Clone, Data)]
pub struct ToolbarWidget {
    fullscreen_button: WidgetPod<ThemedButtonState, ThemedButton>,
    next_button: WidgetPod<ThemedButtonState, ThemedButton>,
    prev_button: WidgetPod<ThemedButtonState, ThemedButton>,
}
impl ToolbarWidget {
    pub fn new() -> Self {
        let fullscreen =
            match include_str!("../resources/buttons/fullscreen.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let fullscreen_hot =
            match include_str!("../resources/buttons/fullscreen_hot.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        let fullscreen_active =
            match include_str!("../resources/buttons/fullscreen_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };

        let next = match include_str!("../resources/buttons/next.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let next_hot = match include_str!("../resources/buttons/next_hot.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let next_active =
            match include_str!("../resources/buttons/next_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };

        let prev = match include_str!("../resources/buttons/prev.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let prev_hot = match include_str!("../resources/buttons/prev_hot.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let prev_active =
            match include_str!("../resources/buttons/prev_active.svg").parse::<SvgData>() {
                Ok(svg) => svg,
                Err(_) => SvgData::default(),
            };
        Self {
            fullscreen_button: WidgetPod::new(ThemedButton::new(
                Size::new(64., 64.),
                fullscreen,
                fullscreen_hot,
                fullscreen_active,
            )),
            next_button: WidgetPod::new(ThemedButton::new(
                Size::new(68., 32.),
                next,
                next_hot,
                next_active,
            )),
            prev_button: WidgetPod::new(ThemedButton::new(
                Size::new(68., 32.),
                prev,
                prev_hot,
                prev_active,
            )),
        }
    }
}

impl Widget<ToolbarState> for ToolbarWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut ToolbarState, _env: &Env) {
        self.fullscreen_button
            .event(_ctx, _event, &mut _data.fullscreen_button, _env);
        self.next_button
            .event(_ctx, _event, &mut _data.next_button, _env);
        self.prev_button
            .event(_ctx, _event, &mut _data.prev_button, _env);

        if _data.next_button.has_event() {
            _data.next_button.clear_event();
            _data.set_right(true);
        } else if _data.prev_button.has_event() {
            _data.prev_button.clear_event();
            _data.set_left(true);
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &ToolbarState,
        _env: &Env,
    ) {
        self.fullscreen_button
            .lifecycle(_ctx, _event, &_data.fullscreen_button, _env);
        self.next_button
            .lifecycle(_ctx, _event, &_data.next_button, _env);
        self.prev_button
            .lifecycle(_ctx, _event, &_data.prev_button, _env);
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &ToolbarState,
        _data: &ToolbarState,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &ToolbarState,
        _env: &Env,
    ) -> Size {
        self.fullscreen_button
            .layout(_layout_ctx, &bc.loosen(), &_data.fullscreen_button, _env);
        let fullscreen_button_origin =
            Point::new(bc.max().width / 2. - 64. / 2., bc.max().height / 2. - 32.);
        self.fullscreen_button.set_origin(
            _layout_ctx,
            &_data.fullscreen_button,
            _env,
            fullscreen_button_origin,
        );

        self.next_button
            .layout(_layout_ctx, &bc.loosen(), &_data.next_button, _env);
        let next_button_origin = Point::new(
            bc.max().width / 2. - 64. / 2. + 54.,
            bc.max().height / 2. - 16.,
        );
        self.next_button
            .set_origin(_layout_ctx, &_data.next_button, _env, next_button_origin);

        self.prev_button
            .layout(_layout_ctx, &bc.loosen(), &_data.prev_button, _env);
        let prev_button_origin = Point::new(
            bc.max().width / 2. - 64. / 2. - 58.,
            bc.max().height / 2. - 16.,
        );
        self.prev_button
            .set_origin(_layout_ctx, &_data.prev_button, _env, prev_button_origin);

        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &ToolbarState, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        let fill_color = Color::rgba(1., 1., 1., 0.8);
        ctx.fill(rect, &fill_color);

        self.fullscreen_button
            .paint(ctx, &data.fullscreen_button, env);
        self.next_button.paint(ctx, &data.next_button, env);
        self.prev_button.paint(ctx, &data.prev_button, env);
    }
}
