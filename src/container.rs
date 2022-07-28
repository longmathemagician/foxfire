use druid::keyboard_types::Key::Character;
use druid::piet::{Image, InterpolationMode, PietImage};

use druid::widget::prelude::*;
use druid::widget::{Button, Click, ControllerHost, Svg, SvgData};
use druid::{KbKey, Point, Target, WidgetExt};
use druid::{Modifiers, Size};
use druid::{WidgetPod};


use crate::app_state::*;
use crate::image_widget::*;
use crate::toolbar_data::*;
use crate::toolbar_widget::*;

use crate::{LOAD_NEW_IMAGE, NEXT_IMAGE, PREV_IMAGE};

// #[derive(Clone, Data)]
pub struct ContainerWidget {
    image_widget: WidgetPod<AppState, ImageWidget>,
    toolbar: WidgetPod<ToolbarState, ToolbarWidget>,
    spinner: WidgetPod<AppState, Svg>,
    spinner_size: Size,
    load_image_button: WidgetPod<AppState, ControllerHost<Button<AppState>, Click<AppState>>>,
    blur_cache: Option<PietImage>,
}

impl ContainerWidget {
    pub fn new() -> Self {
        let spinner_svg_data = match include_str!("../resources/spinner.svg").parse::<SvgData>() {
            Ok(svg) => svg,
            Err(_) => SvgData::default(),
        };
        let spinner_size = spinner_svg_data.size();


        let button_data = String::from("Load image");
        let button = Button::new(button_data).on_click(
            |_ctx, data: &mut AppState, _env| data.show_file_load_dialog());

        Self {
            image_widget: WidgetPod::new(ImageWidget::new()),
            toolbar: WidgetPod::new(ToolbarWidget::new()),
            spinner: WidgetPod::new(Svg::new(spinner_svg_data)),
            spinner_size,
            load_image_button: WidgetPod::new(button),
            blur_cache: None,
        }
    }
}

impl Widget<AppState> for ContainerWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {
        let needs_paint: bool = false;
        let anchor = _data.get_toolbar_state();
        let mut toolbar_state = anchor.lock().unwrap();

        let event_sink = _ctx.get_external_handle();

        if let Event::KeyDown(k) = _event {
            // Key events are always handled here in the container
            if k.key == KbKey::ArrowRight {
                event_sink
                    .submit_command(NEXT_IMAGE, true, Target::Auto)
                    .expect("Failed to send load next image command");
            } else if k.key == KbKey::ArrowLeft {
                event_sink
                    .submit_command(PREV_IMAGE, true, Target::Auto)
                    .expect("Failed to send load previous image command");
            } else if k.key == Character(String::from('o')) && k.mods == Modifiers::CONTROL {
                event_sink
                    .submit_command(LOAD_NEW_IMAGE, true, Target::Auto)
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
                self.toolbar.event(_ctx, _event, &mut toolbar_state, _env);
            }
        } else if let Event::Zoom(_e) = _event {
            _ctx.set_focus(self.image_widget.id());
            self.image_widget.event(_ctx, _event, _data, _env);
            _data.set_image_center_state(false);
        } else if let Event::Internal(_e) = _event {
            self.image_widget.event(_ctx, _event, _data, _env);
            self.toolbar.event(_ctx, _event, &mut toolbar_state, _env);
        } else if let Event::WindowConnected = _event {
            _data.set_window_readiness(true);
        } else if let Event::WindowSize(_e) = _event {} else {
            self.image_widget.event(_ctx, _event, _data, _env);
            self.toolbar.event(_ctx, _event, &mut toolbar_state, _env);
        }


        if needs_paint == true {
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

        self.spinner.lifecycle(_ctx, _event, _data, _env);

        self.load_image_button.lifecycle(_ctx, _event, &_data, _env);
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        let mut needs_paint = true; // repaint on all updates, for now

        if _data.get_image_center_state() && !_old_data.get_image_center_state() {
            self.image_widget
                .widget_mut()
                .update(_ctx, _old_data, _data, _env);
            needs_paint = true;
        }

        if needs_paint == true {
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

        self.spinner.layout(_layout_ctx, &bc.loosen(), _data, _env);
        let spinner_origin = Point::new(
            bc.max().width / 2.0 - self.spinner_size.width / 2.0,
            (bc.max().height - toolbar_height) / 2.0 - self.spinner_size.height / 2.0,
        );
        self.spinner
            .set_origin(_layout_ctx, _data, _env, spinner_origin);

        self.load_image_button.layout(_layout_ctx, &bc.loosen(), &_data, _env);
        let load_image_button_origin = Point::new(
            bc.max().width / 2.0 - self.spinner_size.width / 2.0,
            (bc.max().height - toolbar_height) / 2.0 - self.spinner_size.height / 2.0,
        );
        self.load_image_button
            .set_origin(_layout_ctx, &_data, _env, spinner_origin);

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

        let is_full_paint = if let Some(clip_rect) = final_region {
            if clip_rect.width().ceil() == container_size.width.ceil()
                && clip_rect.height().ceil() == container_size.height.ceil()
            {
                true // Context has a full size clip region
            } else {
                false // Context's clip region is smaller than the context
            }
        } else {
            true // Context lacks a clip region
        };

        let container_alignment_offset = 0.01;
        let toolbar_blur_region_rect = druid::Rect::new(
            0.,
            container_size.height - data.get_toolbar_height() + container_alignment_offset,
            container_size.width,
            container_size.height - container_alignment_offset,
        );

        let anchor = data.get_toolbar_state();
        let toolbar_state = anchor.lock().unwrap();
        let paint_region = match final_region {
            Some(rect) => *rect,
            _ => container_size.to_rect(),
        };

        self.image_widget.paint(ctx, data, env);

        if data.get_loading_state() {
            self.spinner.paint(ctx, data, env);
        } else if !data.has_image() {
            self.load_image_button.paint(ctx, &data, env);
        }

        if is_full_paint {
            let capture_result = ctx.capture_image_area(toolbar_blur_region_rect);
            if let Ok(captured_image) = capture_result {
                let blur_result = ctx.blur_image(&captured_image, 30.);
                if let Ok(blurred_image) = blur_result {
                    ctx.draw_image(
                        &blurred_image,
                        toolbar_blur_region_rect,
                        InterpolationMode::Bilinear,
                    );

                    self.blur_cache = Some(blurred_image)
                }
            }
        } else if let Some(cached_image) = &self.blur_cache {
            ctx.draw_image(
                cached_image,
                toolbar_blur_region_rect,
                InterpolationMode::Bilinear,
            );
        }

        ctx.with_child_ctx(paint_region, move |h| {
            h.with_child_ctx(paint_region, move |i| {
                self.toolbar.paint(i, &toolbar_state, env);
            });
        });
    }
}
