use crate::data::*;
use crate::events::*;
use crate::image_container::*;
use crate::types::*;
use druid::kurbo::BezPath;
use druid::piet::{Brush, FontFamily, ImageFormat, InterpolationMode, Text, TextLayoutBuilder};
use druid::widget::prelude::*;
use druid::{
    Affine, AppLauncher, Color, Cursor, FontDescriptor, LocalizedString, Point, Rect, TextLayout,
    WindowDesc,
};
use image::{DynamicImage, ImageBuffer};
use std::sync::Arc;

#[derive(Clone, Data)]
pub struct ImageWidget;
impl ImageWidget {
    fn map_f64(value: f64, in_l: f64, in_u: f64, out_l: f64, out_u: f64) -> f64 {
        let result = out_l + (value - in_l) * (out_u - out_l) / (in_u - in_l);
        result
    }

    fn get_return_size(image: Size, container: Size, current_zoom: f64) -> Size {
        let image_aspect_ratio = image.width / image.height;
        let container_aspect_ratio = container.width / container.height;

        let target_width: f64;
        let target_height: f64;

        if container_aspect_ratio > image_aspect_ratio {
            // the container is wider than the image
            target_height = image.height / current_zoom;
            target_width = target_height * container_aspect_ratio;
        } else {
            // the container is taller than the image
            target_width = image.width / current_zoom;
            target_height = target_width / container_aspect_ratio;
        }
        Size::new(target_width, target_height)
    }
    fn translate_rect(target: Rect, translation: Position) -> Rect {
        Rect::new(
            target.x0 + translation.x,
            target.y0 + translation.y,
            target.x1 + translation.x,
            target.y1 + translation.y,
        )
    }
    fn scale_rect(target: Rect, scale_factor: f64) -> Rect {
        let distance_to_origin: Position = Position::new(-target.x0, -target.y0);
        let target_at_origin = ImageWidget::translate_rect(target, distance_to_origin);
        let target_scaled = Rect::new(
            0.,
            0.,
            target_at_origin.x1 / scale_factor,
            target_at_origin.y1 / scale_factor,
        );
        let distance_back: Position =
            Position::new(target.x0 / scale_factor, target.y0 / scale_factor);
        let target_scaled_offset: Rect = ImageWidget::translate_rect(target_scaled, distance_back);
        target_scaled_offset
    }
    fn scale_rect_at_position(target: Rect, scale_factor: f64, zoom_point: Position) -> Rect {
        let distance_to_origin: Position = Position::new(-zoom_point.x, -zoom_point.y);
        let target_at_origin = ImageWidget::translate_rect(target, distance_to_origin);
        let target_scaled = ImageWidget::scale_rect(target_at_origin, scale_factor);
        let target_scaled_offset: Rect = ImageWidget::translate_rect(target_scaled, zoom_point);
        target_scaled_offset
    }
    fn get_rect_center(rect: Rect) -> Position {
        Position::new(
            rect.x0 - (rect.x0 - rect.x1) / 2.,
            rect.y0 - (rect.y0 - rect.y1) / 2.,
        )
    }
}

impl Widget<AppState> for ImageWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {
        let mut anchor = _data.get_image_ref();
        let mut image_container = anchor.lock().unwrap();
        if let Event::Wheel(mouse_event) = _event {
            if image_container.event_queue.is_none() {
                let mouse_position =
                    Position::new(mouse_event.window_pos.x, mouse_event.window_pos.y);
                image_container.event_queue = Some(MouseEvent::Zoom(ZoomEvent::new(
                    mouse_event.wheel_delta.y,
                    mouse_position,
                )));
            }
            _ctx.request_paint();
        } else if let Event::MouseDown(mouse_event) = _event {
            if image_container.event_queue.is_none() {
                let mouse_pos = Position::new(mouse_event.window_pos.x, mouse_event.window_pos.y);
                if mouse_event.button.is_left() {
                    let new_drag_event = DragEvent::new(mouse_pos, false);
                    image_container.event_queue = Some(MouseEvent::Drag(new_drag_event));
                    // _ctx.set_cursor(&Cursor::Crosshair);
                } else if mouse_event.button.is_right() {
                    let click_event = ClickEvent::new(mouse_pos);
                    image_container.event_queue = Some(MouseEvent::Click(click_event));
                }
            }
            _ctx.request_paint();
        } else if let Event::MouseMove(mouse_event) = _event {
            if let Some(MouseEvent::Drag(drag_event)) = &mut image_container.event_queue {
                if !drag_event.is_finished() {
                    let current_pos =
                        Position::new(mouse_event.window_pos.x, mouse_event.window_pos.y);
                    drag_event.set_delta(current_pos);
                    _ctx.request_paint();
                }
            }
        } else if let Event::MouseUp(_mouse_event) = _event {
            if let Some(active_event) = &mut image_container.event_queue {
                if let MouseEvent::Drag(drag_event) = active_event {
                    drag_event.complete();
                }
                _ctx.request_paint();
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
        let mut anchor = _data.get_image_ref();
        let mut image_container = anchor.lock().unwrap();
        let mut event_queue = &mut image_container.event_queue;
        if let LifeCycle::WidgetAdded = _event {
            let mut new_title = "🐧 Photo Viewer - ".to_string();
            // new_title.push_str(&self.image_path);
            _ctx.window().set_title(&new_title);
        }
        if let LifeCycle::FocusChanged(false) | LifeCycle::HotChanged(false) = _event {
            if let Some(MouseEvent::Drag(drag_event)) = &mut event_queue {
                drag_event.complete();
            }
        }
    }

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &AppState, _data: &AppState, _env: &Env) {
        println!("UPDATE");
        let mut anchor = _data.get_image_ref();
        let mut image_container = anchor.lock().unwrap();
        if image_container.event_queue.is_some() {
            if let Some(MouseEvent::Drag(drag_event)) = &mut image_container.event_queue {
                if drag_event.is_finished() {
                    _ctx.set_cursor(&Cursor::Arrow);
                } else if drag_event.is_new() {
                    drag_event.mark_seen();
                    _ctx.set_cursor(&Cursor::Crosshair);
                }
            }
            // _ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _layout_ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(100.0, 100.0);
            bc.constrain(size)
        }
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &AppState, env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        let mut anchor = data.get_image_ref();
        let mut image_container = anchor.lock().unwrap();
        if !image_container.has_cache() {
            let cached_image_size = image_container.get_size();
            let image_result = ctx.make_image(
                cached_image_size.width as usize,
                cached_image_size.height as usize,
                image_container.get_image().as_bytes(),
                ImageFormat::Rgb,
            );
            image_container.set_cache(image_result.unwrap());
        }
        let image_size = image_container.get_size();
        //
        let mut drag_position_delta: Option<Position> = None;
        let mut save_drag_position: bool = false;
        let mut zoom_target: Option<Position> = None;
        let mut zoom_factor: f64 = 0.;

        // println!("{:#?}", image_container.event_queue);

        if let Some(MouseEvent::Drag(drag_event)) = &image_container.event_queue {
            drag_position_delta = Some(drag_event.get_delta());
            if drag_event.is_finished() {
                save_drag_position = true;
                image_container.event_queue = None;
            }
        } else if let Some(MouseEvent::Zoom(zoom_event)) = &image_container.event_queue {
            zoom_factor = -zoom_event.get_magnitude() / 1000.;
            zoom_target = Some(zoom_event.get_position());
            image_container.event_queue = None;
        } else if let Some(MouseEvent::Click(click_event)) = &image_container.event_queue {
            image_container.event_queue = None;
        }
        //
        let image: Size = image_size;
        let container: Size = size;
        let drag_delta = drag_position_delta;
        let save_drag_delta = save_drag_position;
        let click_pos = zoom_target;
        let zoom_delta = zoom_factor;

        //
        let default_zoom: f64 = image_container.transform.get_zoom_factor();
        let image_viewport = ImageWidget::get_return_size(image, container, default_zoom);

        let mut drag_center = image_container.transform.get_drag_position();
        if drag_delta.is_some() {
            let drag_delta_image_space: Position = Position::new(
                -ImageWidget::map_f64(
                    drag_delta.unwrap().x(),
                    0.,
                    container.width,
                    0.,
                    image_viewport.width,
                ),
                -ImageWidget::map_f64(
                    drag_delta.unwrap().y(),
                    0.,
                    container.height,
                    0.,
                    image_viewport.height,
                ),
            );
            drag_center += drag_delta_image_space;
            if save_drag_delta {
                image_container.transform.set_drag_position(drag_center);
            }
        }

        let mut output_viewport: Rect =
            Rect::new(0., 0., image_viewport.width, image_viewport.height);
        let centering_offset =
            Position::new(-image_viewport.width / 2., -image_viewport.height / 2.);
        output_viewport = ImageWidget::translate_rect(output_viewport, centering_offset);
        output_viewport = ImageWidget::translate_rect(output_viewport, drag_center);
        if click_pos.is_some() {
            let click_position_image_space = Position::new(
                ImageWidget::map_f64(
                    click_pos.unwrap().x,
                    0.,
                    container.width,
                    0.,
                    image_viewport.width,
                ) + (drag_center.x - image_viewport.width / 2.),
                ImageWidget::map_f64(
                    click_pos.unwrap().y,
                    0.,
                    container.height,
                    0.,
                    image_viewport.height,
                ) + (drag_center.y - image_viewport.height / 2.),
            );

            let mut delta_zoom_factor: f64 = 1. + zoom_delta;
            let zoom_factor = image_container.transform.get_zoom_factor() * delta_zoom_factor;
            if zoom_factor > 100. {
                delta_zoom_factor = 1.;
            } else if zoom_factor < 0.5 {
                delta_zoom_factor = 1.
            } else {
                image_container.transform.set_zoom_factor(zoom_factor);
            }

            let output_viewport_scaled = ImageWidget::scale_rect_at_position(
                output_viewport,
                delta_zoom_factor,
                click_position_image_space,
            );
            let offset: Position = ImageWidget::get_rect_center(output_viewport_scaled)
                - ImageWidget::get_rect_center(output_viewport);
            image_container
                .transform
                .set_drag_position(drag_center + offset);

            output_viewport = output_viewport_scaled;
        }

        ctx.draw_image_area(
            image_container.get_cache(),
            output_viewport,
            rect,
            InterpolationMode::NearestNeighbor,
        );
    }
}
