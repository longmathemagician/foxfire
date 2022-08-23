use druid::keyboard_types::Key::Character;
use druid::kurbo::RoundedRect;
use druid::piet::{InterpolationMode, PietImage};
use std::time::Instant;

use druid::widget::prelude::*;

use druid::{Color, Rect, WidgetPod};
use druid::{KbKey, Point, Target};
use druid::{Modifiers, Size};

use crate::app_state::*;
use crate::commands::REDRAW_IMAGE;
use crate::image_widget::*;
use crate::toolbar_widget::*;

use crate::osd_widget::{OSDPayload, OSDWidget};
use crate::{LOAD_NEW_IMAGE, NEXT_IMAGE, PREV_IMAGE};

// #[derive(Clone, Data)]
pub struct ContainerWidget {
    image_widget: WidgetPod<AppState, ImageWidget>,
    toolbar_widget: WidgetPod<AppState, ToolbarWidget>,
    osd_widget: WidgetPod<AppState, OSDWidget>,
    blur_cache: Option<PietImage>,
}

impl ContainerWidget {
    pub fn new() -> Self {
        Self {
            image_widget: WidgetPod::new(ImageWidget::new()),
            toolbar_widget: WidgetPod::new(ToolbarWidget::new()),
            osd_widget: WidgetPod::new(OSDWidget::new(Size::new(256., 64.))),
            blur_cache: None,
        }
    }

    fn paint_osd(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        // let mut container_size = ctx.size();
        // container_size.height -= data.get_toolbar_height();
        // let container_rect = container_size.to_rect().inset(11.);
        // let osd_size = self.osd_widget.widget().get_size();
        // let osd_rect = Rect::from_center_size(container_rect.center(), osd_size);
        // let osd_rect_rounded = RoundedRect::from_rect(osd_rect, 10.);
        // ctx.with_save(|ctx| {
        //     ctx.clip(osd_rect_rounded);
        //     let osd_blur_capture = ctx.capture_image_area(osd_rect);
        //     if let Ok(osd_background_image) = osd_blur_capture {
        //         let osd_blurred_background_result = ctx.blur_image(&osd_background_image, 15.);
        //         if let Ok(osd_blurred_background) = osd_blurred_background_result {
        //             ctx.draw_image(
        //                 &osd_blurred_background,
        //                 osd_rect,
        //                 InterpolationMode::Bilinear,
        //             );
        //         }
        //     }
        // });
        //
        // self.osd_widget.paint(ctx, data, env);
    }
}

impl Widget<AppState> for ContainerWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {
        let event_sink = _ctx.get_external_handle();

        if let Event::Command(cmd) = _event {
            if cmd.get(REDRAW_IMAGE).is_some() {
                _ctx.request_update();
            }
        } else if let Event::KeyDown(k) = _event {
            // Key events are always handled here in the container
            if k.key == KbKey::ArrowRight {
                event_sink
                    .submit_command(NEXT_IMAGE, Instant::now(), Target::Auto)
                    .expect("Failed to send load next image command");
            } else if k.key == KbKey::ArrowLeft {
                event_sink
                    .submit_command(PREV_IMAGE, Instant::now(), Target::Auto)
                    .expect("Failed to send load previous image command");
            } else if k.key == Character(String::from('o')) && k.mods == Modifiers::CONTROL {
                event_sink
                    .submit_command(LOAD_NEW_IMAGE, Instant::now(), Target::Auto)
                    .expect("Failed to send load new image command");
            }
        } else if let Event::MouseDown(e)
        | Event::MouseUp(e)
        | Event::MouseMove(e)
        | Event::Wheel(e) = _event
        {
            if !_data.has_image() {
                self.osd_widget.event(_ctx, _event, _data, _env);
            }

            // Mouse events will be handled by either the toolbar or the image widget
            if e.window_pos.y < _ctx.size().height - _data.get_toolbar_height() {
                _ctx.set_focus(self.image_widget.id());
                self.image_widget.event(_ctx, _event, _data, _env);
                _data.set_image_center_state(false);
            } else {
                _ctx.set_focus(self.toolbar_widget.id());
                self.toolbar_widget.event(_ctx, _event, _data, _env);
            }
        } else if let Event::Zoom(_e) = _event {
            _ctx.set_focus(self.image_widget.id());
            self.image_widget.event(_ctx, _event, _data, _env);
            _data.set_image_center_state(false);
        } else if let Event::Internal(_e) = _event {
            self.image_widget.event(_ctx, _event, _data, _env);
            self.toolbar_widget.event(_ctx, _event, _data, _env);
        } else if let Event::WindowConnected = _event {
        } else if let Event::WindowSize(_e) = _event {
        } else {
            self.image_widget.event(_ctx, _event, _data, _env);
            self.toolbar_widget.event(_ctx, _event, _data, _env);
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

        self.toolbar_widget.lifecycle(_ctx, _event, _data, _env);

        self.osd_widget.lifecycle(_ctx, _event, _data, _env);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        let mut needs_paint = true; // repaint on all updates, for now

        if _data.get_image_center_state() && !_old_data.get_image_center_state() {
            self.image_widget
                .widget_mut()
                .update(_ctx, _old_data, _data, _env);
            needs_paint = true;
        }

        if needs_paint {
            let new_window_title = String::from("Foxfire - ") + &_data.get_image_name();
            _ctx.window().set_title(&new_window_title);
            self.blur_cache = None;
            _ctx.children_changed();
            _ctx.request_paint();
        }
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

        self.toolbar_widget
            .layout(_layout_ctx, &toolbar_layout, _data, _env);
        self.toolbar_widget.set_origin(
            _layout_ctx,
            _data,
            _env,
            Point::new(0.0, bc.max().height - toolbar_height),
        );

        self.osd_widget
            .layout(_layout_ctx, &bc.loosen(), _data, _env);
        let osd_widget_size = self.osd_widget.widget().get_size();
        let osd_widget_origin = Point::new(
            bc.max().width / 2.0 - osd_widget_size.width / 2.0,
            (bc.max().height - toolbar_height) / 2.0 - osd_widget_size.height / 2.0,
        );
        self.osd_widget
            .set_origin(_layout_ctx, _data, _env, osd_widget_origin);

        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let container_size = ctx.size();

        let final_region = ctx.region().rects().last();
        let region_count: usize = ctx.region().rects().len();

        let is_full_paint = if let Some(clip_rect) = final_region {
            clip_rect.width().ceil() == container_size.width.ceil()
                && clip_rect.height().ceil() == container_size.height.ceil()
                && region_count > 0
        } else {
            false
        };

        // Always paint background & toolbar
        let container_alignment_offset = if cfg!(windows) { 0.01 } else { 0.0 };
        let toolbar_blur_region_rect = druid::Rect::new(
            0.,
            container_size.height - data.get_toolbar_height() + container_alignment_offset,
            container_size.width,
            container_size.height - container_alignment_offset,
        );

        self.image_widget.paint(ctx, data, env);

        if is_full_paint {
            let capture_result = ctx.capture_image_area(toolbar_blur_region_rect);
            if let Ok(captured_image) = capture_result {
                let blur_result = ctx.blur_image(&captured_image, 15.);
                if let Ok(blurred_image) = blur_result {
                    ctx.draw_image(
                        &blurred_image,
                        toolbar_blur_region_rect,
                        InterpolationMode::Bilinear,
                    );
                    self.blur_cache = Some(blurred_image);
                }
            }
        } else if let Some(cached_image) = &self.blur_cache {
            ctx.draw_image(
                cached_image,
                toolbar_blur_region_rect,
                InterpolationMode::Bilinear,
            );
        }

        self.toolbar_widget.paint(ctx, data, env);

        // Paint the load button if there is no loaded image and we're not loading
        if !data.has_image() && !data.get_loading_state() {
            let stroke_color = Color::rgb8(136, 192, 208);
            let load_file_payload = OSDPayload::new(
                Some(LOAD_NEW_IMAGE),
                "Open file".to_string(),
                20.,
                stroke_color,
            );
            self.osd_widget.widget_mut().set_payload(load_file_payload);
            self.paint_osd(ctx, data, env);
        }
        // If we're loading an image, paint the loading display
        else if data.get_loading_state() {
            let stroke_color = Color::rgb8(163, 190, 140);
            let load_file_payload =
                OSDPayload::new(None, "Loading image...".to_string(), 20., stroke_color);
            self.osd_widget.widget_mut().set_payload(load_file_payload);
            self.paint_osd(ctx, data, env);
        }
        // If the current image is in the process of being rotated, indicate it
        else if data.get_rotating_state() {
            let stroke_color = Color::rgb8(180, 142, 173);
            let load_file_payload =
                OSDPayload::new(None, "Rotating image...".to_string(), 20., stroke_color);
            self.osd_widget.widget_mut().set_payload(load_file_payload);
            self.paint_osd(ctx, data, env);
        }
        // If the current image is not able to be displayed, indicate as such
        else if data.has_image_error() {
            let stroke_color = Color::rgb8(235, 203, 139);
            let load_file_payload = OSDPayload::new(
                None,
                "Error: failed to load image".to_string(),
                20.,
                stroke_color,
            );
            self.osd_widget.widget_mut().set_payload(load_file_payload);
            self.paint_osd(ctx, data, env);
        }
    }
}
