use druid::widget::prelude::*;
use druid::widget::{Svg, SvgData};
use druid::{MouseButton, Point, Selector, Target, WidgetPod};

pub struct ThemedButton {
    command: Selector<bool>,
    size: Size,
    offset: Point,
    image: WidgetPod<bool, Svg>,
    image_hot: WidgetPod<bool, Svg>,
    image_active: WidgetPod<bool, Svg>,
    mask: Vec<u8>,
    is_hot: bool,
    is_pressed: bool,
}
impl ThemedButton {
    pub fn new(
        command: Selector<bool>,
        size: Size,
        offset: Point,
        image: &str,
        image_hot: &str,
        image_pressed: &str,
        button_mask: Vec<u8>,
    ) -> Self {
        Self {
            command,
            size,
            offset,
            image: WidgetPod::new(Svg::new(image.parse::<SvgData>().unwrap())),
            image_hot: WidgetPod::new(Svg::new(image_hot.parse::<SvgData>().unwrap())),
            image_active: WidgetPod::new(Svg::new(image_pressed.parse::<SvgData>().unwrap())),
            mask: button_mask,
            is_hot: false,
            is_pressed: false,
        }
    }
    pub fn get_offset(&self) -> Point {
        self.offset
    }
}
impl Widget<bool> for ThemedButton {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut bool, _env: &Env) {
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
            if self.mask[y * ((self.size.width - 1.) as usize) + x] == 1 {
                if !self.is_hot {
                    self.is_hot = true;
                    _ctx.request_paint();
                }
            } else if self.is_hot {
                self.is_hot = false;
                _ctx.request_paint();
            }
        }
        if self.is_hot {
            if let Event::MouseDown(m) = _event {
                if m.button == MouseButton::Left {
                    self.is_pressed = true;
                    _ctx.request_paint();
                }
            } else if let Event::MouseUp(m) = _event {
                if m.button == MouseButton::Left && self.is_pressed {
                    let event_sink = _ctx.get_external_handle();
                    event_sink
                        .submit_command(self.command, true, Target::Auto)
                        .expect("Failed to send command");
                    self.is_pressed = false;
                    _ctx.request_paint();
                }
            }
        } else if self.is_pressed {
            self.is_pressed = false;
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &bool, _env: &Env) {
        if let LifeCycle::WidgetAdded = _event {
            self.image.lifecycle(_ctx, _event, _data, _env);
            self.image_hot.lifecycle(_ctx, _event, _data, _env);
            self.image_active.lifecycle(_ctx, _event, _data, _env);
        }
        if let LifeCycle::FocusChanged(_) | LifeCycle::HotChanged(_) = _event {
            if !_ctx.is_active() || !_ctx.is_hot() {
                self.is_hot = false;
            }
            _ctx.request_paint();
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &bool, _data: &bool, _env: &Env) {}

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &bool,
        _env: &Env,
    ) -> Size {
        self.image.layout(_layout_ctx, &bc.loosen(), _data, _env);
        self.image_hot
            .layout(_layout_ctx, &bc.loosen(), _data, _env);
        self.image_active
            .layout(_layout_ctx, &bc.loosen(), _data, _env);
        self.size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &bool, env: &Env) {
        let is_button_hot = self.is_hot;
        let is_context_hot = ctx.is_hot();
        let paint_region = *ctx
            .region()
            .rects()
            .last()
            .expect("Tried to paint with an invalid clip region");

        ctx.with_child_ctx(paint_region, move |f| {
            if self.is_pressed && is_button_hot {
                self.image_active.paint(f, data, env);
            } else if is_context_hot && is_button_hot {
                self.image_hot.paint(f, data, env);
            } else {
                self.image.paint(f, data, env);
            }
        });
    }
}
