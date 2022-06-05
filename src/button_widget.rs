use crate::button_data::*;
use crate::toolbar_data::*;
use druid::piet::{ImageFormat, InterpolationMode, PietImage};
use druid::widget::prelude::*;
use druid::widget::{Svg, SvgData};
use druid::{Color, Point};
use druid::{Data, WidgetPod};
use std::sync::{Arc, Mutex};

// #[derive(Clone, Data)]
pub struct ThemedButton {
    size: Size,
    image: WidgetPod<ThemedButtonState, Svg>,
    image_hot: WidgetPod<ThemedButtonState, Svg>,
    image_active: WidgetPod<ThemedButtonState, Svg>,
}
impl ThemedButton {
    pub fn new(size: Size, image: SvgData, image_hot: SvgData, image_active: SvgData) -> Self {
        Self {
            size,
            image: WidgetPod::new(Svg::new(image)),
            image_hot: WidgetPod::new(Svg::new(image_hot)),
            image_active: WidgetPod::new(Svg::new(image_active)),
        }
    }
}
impl Widget<ThemedButtonState> for ThemedButton {
    fn event(
        &mut self,
        _ctx: &mut EventCtx,
        _event: &Event,
        _data: &mut ThemedButtonState,
        _env: &Env,
    ) {
        if let Event::MouseDown(_) = _event {
            _data.set_pressed(true);
            _ctx.request_paint();
        } else if let Event::MouseUp(_) = _event {
            _data.fire_event();
            _data.set_pressed(false);
            _ctx.request_paint();
        }
    }

    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &ThemedButtonState,
        _env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = _event {
            self.image.lifecycle(_ctx, _event, _data, _env);
            self.image_hot.lifecycle(_ctx, _event, _data, _env);
            self.image_active.lifecycle(_ctx, _event, _data, _env);
        }
        if let LifeCycle::FocusChanged(_) | LifeCycle::HotChanged(_) = _event {
            _ctx.request_paint();
        }
    }

    fn update(
        &mut self,
        _ctx: &mut UpdateCtx,
        _old_data: &ThemedButtonState,
        _data: &ThemedButtonState,
        _env: &Env,
    ) {
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &ThemedButtonState,
        _env: &Env,
    ) -> Size {
        self.image.layout(_layout_ctx, &bc.loosen(), _data, _env);
        self.image_hot
            .layout(_layout_ctx, &bc.loosen(), _data, _env);
        self.image_active
            .layout(_layout_ctx, &bc.loosen(), _data, _env);
        self.size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &ThemedButtonState, env: &Env) {
        if data.is_pressed() {
            self.image_active.paint(ctx, data, env);
        } else if ctx.is_hot() {
            self.image_hot.paint(ctx, data, env);
        } else {
            self.image.paint(ctx, data, env);
        }
    }
}
