use crate::button_data::*;
use crate::toolbar_data::*;
use druid::im::vector::Focus;
use druid::piet::{ImageFormat, InterpolationMode, PietImage};
use druid::widget::prelude::*;
use druid::widget::{Svg, SvgData};
use druid::{Color, Point};
use druid::{Data, WidgetPod};
use std::ops::Index;
use std::sync::{Arc, Mutex};

// #[derive(Clone, Data)]
pub struct ThemedButton {
    is_hot: bool,
    size: Size,
    image: WidgetPod<ThemedButtonState, Svg>,
    image_hot: WidgetPod<ThemedButtonState, Svg>,
    image_active: WidgetPod<ThemedButtonState, Svg>,
    mask: Arc<Vec<bool>>,
}
impl ThemedButton {
    pub fn new(
        size: Size,
        image: SvgData,
        image_hot: SvgData,
        image_active: SvgData,
        mask: Arc<Vec<bool>>,
    ) -> Self {
        Self {
            is_hot: false,
            size,
            image: WidgetPod::new(Svg::new(image)),
            image_hot: WidgetPod::new(Svg::new(image_hot)),
            image_active: WidgetPod::new(Svg::new(image_active)),
            mask: mask,
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
        if let Event::MouseMove(e) = _event {
            let mut x = e.pos.x as usize;
            x = if x > (self.size.width - 1.) as usize {
                (self.size.width - 1.) as usize
            } else {
                x
            };

            let mut y = e.pos.y as usize;
            y = if y > (self.size.height - 1.) as usize {
                (self.size.height - 1.) as usize
            } else {
                y
            };
            if self.mask[y * ((self.size.width - 1.) as usize) + x] {
                if !self.is_hot {
                    self.is_hot = true;
                    _ctx.request_paint();
                }
            } else {
                if self.is_hot {
                    self.is_hot = false;
                    _ctx.request_paint();
                }
            }
        }
        if self.is_hot {
            if let Event::MouseDown(_) = _event {
                _data.set_pressed(true);
                _ctx.request_paint();
            } else if let Event::MouseUp(_) = _event {
                if _data.is_pressed() {
                    _data.fire_event();
                    _data.set_pressed(false);
                    _ctx.request_paint();
                }
            }
        } else if _data.is_pressed() {
            _data.set_pressed(false);
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
        if let LifeCycle::FocusChanged(e) | LifeCycle::HotChanged(e) = _event {
            if !_ctx.is_active() || !_ctx.is_hot() {
                self.is_hot = false;
            }
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
        if data.is_pressed() && self.is_hot {
            self.image_active.paint(ctx, data, env);
        } else if ctx.is_hot() && self.is_hot {
            self.image_hot.paint(ctx, data, env);
        } else {
            self.image.paint(ctx, data, env);
        }
    }
}
