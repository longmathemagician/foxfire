use druid::keyboard_types::Key::Character;
use druid::kurbo::{RoundedRect, Shape};
use druid::piet::{InterpolationMode, PietImage, Text, TextLayout, TextLayoutBuilder};
use std::time::Instant;

use druid::widget::prelude::*;
use druid::widget::{Button, Click, ControllerHost, Svg, SvgData};
use druid::{Color, FontFamily, WidgetPod};
use druid::{KbKey, Point, Target};
use druid::{Modifiers, Size};

use crate::app_state::*;
use crate::commands::REDRAW_IMAGE;
use crate::image_widget::*;
use crate::toolbar_widget::*;

use crate::{LOAD_NEW_IMAGE, NEXT_IMAGE, PREV_IMAGE};

// #[derive(Clone, Data)]
pub struct ContainerWidget {
    image_widget: WidgetPod<AppState, ImageWidget>,
    toolbar: WidgetPod<AppState, ToolbarWidget>,
    spinner: WidgetPod<AppState, Svg>,
    spinner_size: Size,
    load_image_button: WidgetPod<AppState, ControllerHost<Button<AppState>, Click<AppState>>>,
    blur_cache: Option<PietImage>,
}

impl ContainerWidget {
    pub fn new() -> Self {
        let spinner_svg_data = include_str!("../resources/spinner.svg")
            .parse::<SvgData>()
            .unwrap();
        let spinner_size = spinner_svg_data.size();

        let button_data = String::from("Load image");
        let button = Button::new(button_data)
            .on_click(|_ctx, data: &mut AppState, _env| data.show_file_load_dialog());

        Self {
            image_widget: WidgetPod::new(ImageWidget::new()),
            toolbar: WidgetPod::new(ToolbarWidget::new()),
            spinner: WidgetPod::new(Svg::new(spinner_svg_data)),
            spinner_size,
            load_image_button: WidgetPod::new(button),
            blur_cache: None,
        }
    }

    fn paint_widgets(
        &mut self,
        container_size: Size,
        is_full_paint: bool,
        ctx: &mut PaintCtx,
        data: &AppState,
        env: &Env,
    ) {
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

        self.toolbar.paint(ctx, data, env);
    }
    fn paint_osd(
        &mut self,
        ctx: &mut PaintCtx,
        data: &AppState,
        text_contents: String,
        text_size: f64,
        stroke_color: Color,
    ) {
        let (text_color, fill_color) = if data.dark_theme_enabled {
            (Color::rgb8(255, 255, 255), Color::rgba(0.2, 0.2, 0.2, 0.5))
        } else {
            (Color::rgb8(0, 0, 0), Color::rgba(1., 1., 1., 0.5))
        };
        let container_size = ctx.size();
        let text_handler = ctx.text();
        let layout = text_handler
            .new_text_layout(text_contents)
            .font(FontFamily::SYSTEM_UI, text_size)
            .text_color(text_color)
            .build()
            .unwrap();
        let text_bounds = layout.image_bounds();
        let osd_size = text_bounds.expand().inflate(25., 25.);

        let osd_rect = druid::Rect::new(
            container_size.width / 2. - osd_size.width() / 2.,
            (container_size.height - data.get_toolbar_height()) / 2. - osd_size.height() / 2.,
            container_size.width / 2. + osd_size.width() / 2.,
            (container_size.height - data.get_toolbar_height()) / 2. + osd_size.height() / 2.,
        );
        let osd_rrect = RoundedRect::from_rect(osd_rect, 10.0);

        ctx.with_save(|ctx| {
            ctx.clip(osd_rrect);

            let osd_blur_capture = ctx.capture_image_area(osd_rect);
            if let Ok(osd_background_image) = osd_blur_capture {
                let osd_blurred_background_result = ctx.blur_image(&osd_background_image, 15.);
                if let Ok(osd_blurred_background) = osd_blurred_background_result {
                    ctx.draw_image(
                        &osd_blurred_background,
                        osd_rect,
                        InterpolationMode::Bilinear,
                    );
                }
            }

            ctx.fill(osd_rect, &fill_color);
            ctx.stroke(osd_rrect.into_path(0.5), &stroke_color, 4.);

            let text_point = Point::new(
                osd_rrect.center().x - osd_size.center().x,
                osd_rrect.center().y - osd_size.center().y,
            );
            ctx.draw_text(&layout, text_point);
        });
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
                self.load_image_button.event(_ctx, _event, _data, _env);
            }

            // Mouse events will be handled by either the toolbar or the image widget
            if e.window_pos.y < _ctx.size().height - _data.get_toolbar_height() {
                _ctx.set_focus(self.image_widget.id());
                self.image_widget.event(_ctx, _event, _data, _env);
                _data.set_image_center_state(false);
            } else {
                _ctx.set_focus(self.toolbar.id());
                self.toolbar.event(_ctx, _event, _data, _env);
            }
        } else if let Event::Zoom(_e) = _event {
            _ctx.set_focus(self.image_widget.id());
            self.image_widget.event(_ctx, _event, _data, _env);
            _data.set_image_center_state(false);
        } else if let Event::Internal(_e) = _event {
            self.image_widget.event(_ctx, _event, _data, _env);
            self.toolbar.event(_ctx, _event, _data, _env);
        } else if let Event::WindowConnected = _event {
        } else if let Event::WindowSize(_e) = _event {
        } else {
            self.image_widget.event(_ctx, _event, _data, _env);
            self.toolbar.event(_ctx, _event, _data, _env);
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

        self.toolbar.lifecycle(_ctx, _event, _data, _env);

        self.spinner.lifecycle(_ctx, _event, _data, _env);

        self.load_image_button.lifecycle(_ctx, _event, _data, _env);
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

        self.toolbar
            .layout(_layout_ctx, &toolbar_layout, _data, _env);
        self.toolbar.set_origin(
            _layout_ctx,
            _data,
            _env,
            Point::new(0.0, bc.max().height - toolbar_height),
        );

        self.spinner.layout(_layout_ctx, &bc.loosen(), _data, _env);
        let spinner_origin = Point::new(
            bc.max().width / 2.0 - self.spinner_size.width / 2.0,
            (bc.max().height - toolbar_height) / 2.0 - self.spinner_size.height / 2.0,
        );
        self.spinner
            .set_origin(_layout_ctx, _data, _env, spinner_origin);

        self.load_image_button
            .layout(_layout_ctx, &bc.loosen(), _data, _env);
        let load_image_button_origin = Point::new(
            bc.max().width / 2.0 - self.spinner_size.width / 2.0,
            (bc.max().height - toolbar_height) / 2.0 - self.spinner_size.height / 2.0,
        );
        self.load_image_button
            .set_origin(_layout_ctx, _data, _env, load_image_button_origin);

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
        self.paint_widgets(container_size, is_full_paint, ctx, data, env);

        // Paint the load button if there is no loaded image and we're not loading
        if !data.has_image() && !data.get_loading_state() {
            self.load_image_button.paint(ctx, data, env);
        }
        // If we're loading an image, paint the loading display
        else if data.get_loading_state() {
            let stroke_color = if data.dark_theme_enabled {
                Color::rgb8(29, 14, 8)
            } else {
                Color::rgb8(136, 192, 208)
            };
            self.paint_osd(ctx, data, "Loading image...".to_string(), 20., stroke_color);
        }
        // If the current image is not able to be displayed, indicate as such
        else if data.has_image_error() {
            let stroke_color = Color::rgb8(235, 203, 139);
            self.paint_osd(
                ctx,
                data,
                "Error: failed to load image".to_string(),
                20.,
                stroke_color,
            );
        }
    }
}
