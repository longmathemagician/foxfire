use druid::widget::prelude::*;
use druid::widget::{Svg, SvgData};
use druid::{MouseButton, Point, Selector, Target, WidgetPod};
use std::time::Instant;

pub struct ThemedButton {
    command: Option<Selector<Instant>>,
    size: Size,
    offset: Point,
    image: WidgetPod<bool, Svg>,
    image_hot: WidgetPod<bool, Svg>,
    image_active: WidgetPod<bool, Svg>,
    image_disabled: WidgetPod<bool, Svg>,
    mask: Vec<u8>,
    is_hot: bool,
    is_pressed: bool,
    is_enabled: bool,
}

impl ThemedButton {
    pub fn new(
        command: Option<Selector<Instant>>,
        size: Size,
        offset: Point,
        image: &str,
        image_hot: &str,
        image_active: &str,
        image_disabled: &str,
        button_mask: Vec<u8>,
    ) -> Self {
        Self {
            command,
            size,
            offset,
            image: WidgetPod::new(Svg::new(image.parse::<SvgData>().unwrap())),
            image_hot: WidgetPod::new(Svg::new(image_hot.parse::<SvgData>().unwrap())),
            image_active: WidgetPod::new(Svg::new(image_active.parse::<SvgData>().unwrap())),
            image_disabled: WidgetPod::new(Svg::new(image_disabled.parse::<SvgData>().unwrap())),
            mask: button_mask,
            is_hot: false,
            is_pressed: false,
            is_enabled: matches!(command, Some(_)),
        }
    }
    pub fn get_offset(&self) -> Point {
        self.offset
    }
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }
    pub fn disable(&mut self) {
        self.is_enabled = false;
    }
}

impl Widget<bool> for ThemedButton {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut bool, _env: &Env) {
        if !self.is_enabled {
            return;
        }

        let mut event_handled = false;
        let mut needs_repaint = false;

        if let Some(command) = self.command {
            if let Event::MouseMove(e) = event {
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

                        event_handled = true;
                        needs_repaint = true;
                    }
                } else if self.is_hot {
                    self.is_hot = false;

                    event_handled = true;
                    needs_repaint = true;
                }
            }
            if self.is_hot {
                if let Event::MouseDown(m) = event {
                    event_handled = true;
                    if m.button == MouseButton::Left {
                        self.is_pressed = true;

                        needs_repaint = true;
                    }
                } else if let Event::MouseUp(m) = event {
                    event_handled = true;
                    if m.button == MouseButton::Left && self.is_pressed {
                        let event_sink = ctx.get_external_handle();
                        event_sink
                            .submit_command(command, Instant::now(), Target::Auto)
                            .expect("Failed to send command");
                        self.is_pressed = false;

                        needs_repaint = true;
                    }
                }
            } else if self.is_pressed {
                self.is_pressed = false;

                event_handled = true;
            }
        }

        if event_handled {
            ctx.set_handled()
        }
        if needs_repaint {
            ctx.request_paint()
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &bool, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            self.image.lifecycle(ctx, event, data, env);
            self.image_hot.lifecycle(ctx, event, data, env);
            self.image_active.lifecycle(ctx, event, data, env);
            self.image_disabled.lifecycle(ctx, event, data, env);
        }
        if let LifeCycle::FocusChanged(_) | LifeCycle::HotChanged(_) = event {
            if !ctx.is_active() || !ctx.is_hot() {
                self.is_hot = false;
            }
            ctx.request_paint();
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &bool, _data: &bool, _env: &Env) {}

    fn layout(
        &mut self,
        layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &bool,
        env: &Env,
    ) -> Size {
        self.image.layout(layout_ctx, &bc.loosen(), data, env);
        self.image_hot.layout(layout_ctx, &bc.loosen(), data, env);
        self.image_active
            .layout(layout_ctx, &bc.loosen(), data, env);
        self.image_disabled
            .layout(layout_ctx, &bc.loosen(), data, env);
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
            if !self.is_enabled || self.command.is_none() {
                self.image_disabled.paint(f, data, env);
            } else if self.is_pressed && is_button_hot {
                self.image_active.paint(f, data, env);
            } else if is_context_hot && is_button_hot {
                self.image_hot.paint(f, data, env);
            } else {
                self.image.paint(f, data, env);
            }
        });
    }
}
