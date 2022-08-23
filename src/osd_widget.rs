use crate::AppState;
use druid::kurbo::{RoundedRect, Shape};
use druid::piet::{Text, TextLayout, TextLayoutBuilder};
use druid::widget::prelude::*;

use druid::{Color, FontFamily, MouseButton, Point, Rect, Selector, Target};
use std::time::Instant;

pub struct OSDPayload {
    command: Option<Selector<Instant>>,
    text_contents: String,
    text_size: f64,
    stroke_color: Color,
}

impl OSDPayload {
    pub fn new(
        command: Option<Selector<Instant>>,
        text_contents: String,
        text_size: f64,
        stroke_color: Color,
    ) -> Self {
        Self {
            command,
            text_contents,
            text_size,
            stroke_color,
        }
    }
}

pub struct OSDWidget {
    payload: Option<OSDPayload>,
    size: Size,
    is_hot: bool,
    is_pressed: bool,
}

impl OSDWidget {
    pub fn new(size: Size) -> Self {
        Self {
            payload: None,
            size,
            is_hot: false,
            is_pressed: false,
        }
    }
    pub fn set_payload(&mut self, new_payload: OSDPayload) {
        self.payload = Some(new_payload);
    }
    pub fn get_size(&self) -> Size {
        self.size
    }
}

impl Widget<AppState> for OSDWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {
        if let Some(payload) = &self.payload {
            if let Some(button_command) = payload.command {
                if let Event::MouseMove(_e) = _event {
                    if !self.is_hot {
                        self.is_hot = true;
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
                                .submit_command(button_command, Instant::now(), Target::Auto)
                                .expect("Failed to send command");
                            self.is_pressed = false;
                            _ctx.request_paint();
                        }
                    } else if self.is_pressed {
                        self.is_pressed = false;
                    }
                }
            }
        }
    }
    fn lifecycle(
        &mut self,
        _ctx: &mut LifeCycleCtx,
        _event: &LifeCycle,
        _data: &AppState,
        _env: &Env,
    ) {
        if let LifeCycle::FocusChanged(_) | LifeCycle::HotChanged(_) = _event {
            if !_ctx.is_active() || !_ctx.is_hot() {
                self.is_hot = false;
            }
            _ctx.request_paint();
        }
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
        bc.constrain(self.size)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, _env: &Env) {
        let payload: &OSDPayload;
        if let Some(local_payload) = &self.payload {
            payload = local_payload
        } else {
            return;
        }

        let (text_color, fill_color) = if data.dark_theme_enabled {
            (Color::rgb8(255, 255, 255), Color::rgba(0.2, 0.2, 0.2, 0.5))
        } else {
            (Color::rgb8(0, 0, 0), Color::rgba(1., 1., 1., 0.5))
        };
        let text_handler = ctx.text();
        let layout = text_handler
            .new_text_layout(payload.text_contents.clone())
            .font(FontFamily::SYSTEM_UI, payload.text_size)
            .text_color(text_color)
            .build()
            .unwrap();
        let text_bounds = layout.image_bounds();

        let container_rect = ctx.size().to_rect().inset(10.);
        let osd_size = self.size;
        let osd_rect = Rect::from_center_size(container_rect.center(), osd_size);
        let osd_rect_rounded = RoundedRect::from_rect(osd_rect, 10.);

        ctx.with_save(|ctx| {
            ctx.clip(osd_rect_rounded);
            ctx.fill(osd_rect, &fill_color);
            if self.is_pressed {
                let offset = 30;
                let darker_stroke = Color::rgb8(
                    payload.stroke_color.as_rgba8().0 - offset,
                    payload.stroke_color.as_rgba8().1 - offset,
                    payload.stroke_color.as_rgba8().2 - offset,
                );
                ctx.stroke(osd_rect_rounded.into_path(0.5), &darker_stroke, 6.);
            } else if ctx.is_hot() {
                let offset = 30;
                let lighter_stroke = Color::rgb8(
                    payload.stroke_color.as_rgba8().0 + offset,
                    payload.stroke_color.as_rgba8().1 + offset,
                    payload.stroke_color.as_rgba8().2 + offset,
                );
                ctx.stroke(osd_rect_rounded.into_path(0.5), &lighter_stroke, 6.);
            } else {
                ctx.stroke(osd_rect_rounded.into_path(0.5), &payload.stroke_color, 6.);
            }

            let text_point = Point::new(
                osd_rect.center().x - text_bounds.center().x,
                osd_rect.center().y - text_bounds.center().y,
            );
            ctx.draw_text(&layout, text_point);
        });
    }
}
