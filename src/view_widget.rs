use std::sync::mpsc;

use image::{DynamicImage, ImageBuffer};

use druid::piet::{ImageFormat, InterpolationMode, PietImage};
use druid::widget::prelude::*;
use druid::{Color, Point, Rect};
use druid::platform_menus::mac::file::print;



use crate::events::*;
use crate::events::MouseEvent::Click;
use crate::types::*;

pub struct ImageView {
    image_path: String,
    image_src: mpsc::Receiver<DynamicImage>,
    image_data: DynamicImage,
    image_size: Size,
    image_cached: Option<PietImage>,
    event_queue: Option<MouseEvent>,
    transform: ImageTransformation,
}
impl ImageView {
    pub fn new(path: String, source: mpsc::Receiver<DynamicImage>) -> Self {
        ImageView {
            image_path: path,
            image_src: source,
            image_data: DynamicImage::ImageRgb8(ImageBuffer::new(1, 1)),
            image_size: Size::new(1., 1.),
            image_cached: None,
            event_queue: None,
            transform: ImageTransformation::new(),
        }
    }
    fn map_f64(
        value: f64, 
        in_l: f64, in_u: f64, 
        out_l: f64, out_u: f64,
    ) -> f64 {
        let result = out_l + (value - in_l)*(out_u - out_l)/(in_u - in_l);
        result
    }

    fn get_return_size(image: Size, container: Size, current_zoom: f64) -> Size {
        let image_aspect_ratio = image.width / image.height;
        let container_aspect_ratio = container.width / container.height;


        let mut target_width: f64;
        let mut target_height: f64;

        if container_aspect_ratio > image_aspect_ratio { // the container is wider than the image
            target_height = image.height / current_zoom;
            target_width = target_height * container_aspect_ratio;
        } else { // the container is taller than the image
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
        let target_at_origin = ImageView::translate_rect(target, distance_to_origin);
        let target_scaled = Rect::new(
            0.,
            0.,
            target_at_origin.x1 / scale_factor,
            target_at_origin.y1 / scale_factor,
        );
        let distance_back: Position = Position::new(target.x0/scale_factor, target.y0/scale_factor);
        let target_scaled_offset: Rect = ImageView::translate_rect(target_scaled, distance_back);
        target_scaled_offset
    }
    fn scale_rect_at_position(target: Rect, scale_factor: f64, zoom_point: Position) -> Rect {
        // println!("Source rect: {}", target);
        // println!("Scaling around: {}", zoom_point);
        let distance_to_origin: Position = Position::new(-zoom_point.x, -zoom_point.y);
        let target_at_origin = ImageView::translate_rect(target, distance_to_origin);
        let target_scaled = ImageView::scale_rect(target_at_origin, scale_factor);
        // let distance_back: Position = Position::new(target.x0-zoom_point.x*scale_factor, target.y0-zoom_point.y*scale_factor);
        let target_scaled_offset: Rect = ImageView::translate_rect(target_scaled, zoom_point);
        // println!("Returned rect (w/ sf={}): {}", scale_factor, target_scaled_offset);
        target_scaled_offset
    }
    fn get_src_rect(&mut self, image: Size, container: Size, drag_pos: Option<Position>, save_drag: bool, click_pos: Option<Position>, zoom_factor: f64) -> Rect {
        let zoom: f64 = (1.+self.transform.get_zoom_factor());
        let target = ImageView::get_return_size(image, container, zoom);

        if self.transform.viewport.is_none() {

        }
        let mut ctr = self.transform.get_drag_position();
        if drag_pos.is_some() {
            let drag_delta_image_space: Position = Position::new(
                -ImageView::map_f64(drag_pos.unwrap().x(), 0., container.width, 0., target.width),
                -ImageView::map_f64(drag_pos.unwrap().y(), 0., container.height, 0., target.height),
            );
            ctr += drag_delta_image_space;
            if save_drag {
                self.transform.set_drag_position(ctr);
            }
        }

        let mut out: Rect = Rect::new(
            0.,
            0.,
            target.width,
            target.height,
        );
        let centering_offset = Position::new(
            -target.width/2.,
            -target.height/2.,
        );
        out = ImageView::translate_rect(out, centering_offset);
        out = ImageView::translate_rect(out, ctr);
        println!("CTR{}", ctr);
        if click_pos.is_some() {
            // println!("Old image center: {}", ctr);
            let ck_ps_is = Position::new(
                ImageView::map_f64(click_pos.unwrap().x, 0., container.width, 0., target.width) + (ctr.x-target.width/2.),
                ImageView::map_f64(click_pos.unwrap().y, 0., container.height, 0., target.height) + (ctr.y-target.height/2.),
            );

            // println!("Zooming on: {}", ck_ps_is);

            // We now need to center on this point, calculate new bounds, and then rescale them back
            let new_zoom: f64 = (1.+zoom_factor);
            println!("{}", self.transform.get_zoom_factor());

            let ors = ImageView::scale_rect_at_position(out, new_zoom, ck_ps_is);
            self.transform.set_zoom_target(ck_ps_is);
            let offset:Position = Position::new(
                (image.width/2. - ck_ps_is.x) / new_zoom - (image.width/2. - ck_ps_is.x),
                (image.height/2. - ck_ps_is.y) / new_zoom - (image.height/2. - ck_ps_is.y),
            );


            // (image.width/2. - ck_ps_is.x) / new_zoom - (image.width/2. - ck_ps_is.x),
            //     (image.height/2. - ck_ps_is.y) / new_zoom - (image.height/2. - ck_ps_is.y),
            self.transform.set_drag_position(self.transform.get_drag_position() + offset);
            self.transform.zoom_factor *= new_zoom;//BAD
            self.transform.zoom_factor =
                if self.transform.zoom_factor > 10. { 10. }
                else if self.transform.zoom_factor < -0.9 { -0.9 }
                else { self.transform.zoom_factor };
            out = ors;
        }

        out
    }

    fn get_dst_rect(&self, image: Size, container: Size) -> Rect {
        Rect::new(
            0.,
            0.,
            container.width,
            container.height,
        )
    }
}

impl Widget<String> for ImageView {
    fn event(
        &mut self, 
        _ctx: &mut EventCtx, 
        _event: &Event, 
        _data: &mut String, 
        _env: &Env
    ) {
        if let Event::Wheel(mouse_event) = _event {
            if self.event_queue.is_none() {
                let mouse_position = Position::new(
                    mouse_event.window_pos.x, 
                    mouse_event.window_pos.y
                );
                self.event_queue = Some(MouseEvent::Zoom(
                    ZoomEvent::new(mouse_event.wheel_delta.y, mouse_position))
                );
            }
            _ctx.request_update();
        }
        else if let Event::MouseDown(mouse_event) = _event {
            if self.event_queue.is_none() {
                let mouse_pos = Position::new(
                    mouse_event.window_pos.x,
                    mouse_event.window_pos.y,
                );
                if mouse_event.button.is_left() {
                    let new_drag_event = DragEvent::new(mouse_pos, false);
                    self.event_queue = Some(MouseEvent::Drag(new_drag_event));
                } else if mouse_event.button.is_right() {
                    let click_event = ClickEvent::new(mouse_pos);
                    self.event_queue = Some(MouseEvent::Click(click_event));
                }
            }
            _ctx.request_update();
        }
        else if let Event::MouseMove(mouse_event) = _event {
            if let Some(MouseEvent::Drag(drag_event)) = &mut self.event_queue {
                if !drag_event.is_finished() {
                    let current_pos = Position::new(
                        mouse_event.window_pos.x, 
                        mouse_event.window_pos.y,
                    );
                    drag_event.set_delta(current_pos);
                    _ctx.request_update();
                }
            }
        }
        else if let Event::MouseUp(_mouse_event) = _event {
            if let Some(active_event) = &mut self.event_queue {
                if let MouseEvent::Drag(drag_event) = active_event {
                    drag_event.complete();
                }
                _ctx.request_update();
            }
        }
    }
    fn lifecycle(
        &mut self, 
        _ctx: &mut LifeCycleCtx, 
        _event: &LifeCycle, 
        _data: &String, 
        _env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = _event {
            // Receive the image from the thread
            let received_image_handle = self.image_src.recv();
            self.image_data = match received_image_handle {
                Ok(image) => {image},
                Err(_) => DynamicImage::ImageRgb8(ImageBuffer::new(1, 1)),
            };

            self.image_size = Size::new(
                self.image_data.width() as f64, 
                self.image_data.height() as f64,
            );
            let centered_position: Position = Position::new(
                self.image_size.width/2.,
                self.image_size.height/2.,
            );
            self.transform.set_drag_position(centered_position);

            let mut new_title = "Linux Photo Viewer - ".to_string();
            new_title.push_str(&self.image_path);
            _ctx.window().set_title(&new_title);
        }
        if let 
            LifeCycle::FocusChanged(false) 
            | 
            LifeCycle::HotChanged(false) 
        = _event {
            if let Some(MouseEvent::Drag(drag_event)) = &mut self.event_queue {
                drag_event.complete();
            }
        }
    }
    fn update(&mut self, 
        _ctx: &mut UpdateCtx, 
        _old_data: &String, 
        _data: &String, 
        _env: &Env
    ) {
        _ctx.request_paint();
    }
    fn layout(
        &mut self, 
        _layout_ctx: &mut LayoutCtx, 
        bc: &BoxConstraints, 
        _data: &String, 
        _env: &Env,
    ) -> Size {
        if bc.is_width_bounded() && bc.is_height_bounded() {
            bc.max()
        } else {
            let size = Size::new(640., 480.);
            bc.constrain(size)
        }

    }
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &String, _env: &Env) {
        let size = ctx.size();
        let rect = size.to_rect();
        ctx.fill(rect, &Color::WHITE);

        if self.image_cached.is_none() {
            let width: usize = self.image_size.width as usize;
            let height: usize = self.image_size.height as usize;
            let image_result = ctx.make_image(
                width, 
                height, 
                self.image_data.as_bytes(), ImageFormat::Rgb
            );
            self.image_cached = Some(image_result.unwrap());
        }
        let mut tmp_drag_pos: Option<Position> = None;
        let mut save_drag: bool = false;
        let mut tmp_click_pos: Option<Position> = None;
        let mut extra_zoom: f64 = 0.;

        if let Some(MouseEvent::Drag(drag_event)) = &mut self.event_queue {
            tmp_drag_pos = Some(drag_event.get_delta());
            if drag_event.is_finished() {
                save_drag = true;
                self.event_queue = None;
            }
        }
        else if let Some(MouseEvent::Zoom(zoom_event)) = &self.event_queue {
            // self.transform.zoom_factor += 0.;//-zoom_event.get_magnitude()/1000.;
            // self.transform.zoom_factor =
            //     if self.transform.zoom_factor > 10. { 10. }
            //     else if self.transform.zoom_factor < -0.9 { -0.9 }
            //     else { self.transform.zoom_factor };
            extra_zoom = -zoom_event.get_magnitude()/1000.;
            // self.transform.set_zoom_target(zoom_event.get_position());
            tmp_click_pos = Some(zoom_event.get_position());
            self.event_queue = None;
        }
        else if let Some(MouseEvent::Click(click_event)) = &self.event_queue {
            tmp_click_pos = Some(click_event.get_position());
            self.event_queue = None;
        }



        let src_rect = self.get_src_rect(self.image_size, size, tmp_drag_pos, save_drag, tmp_click_pos, extra_zoom);
        let dst_rect = self.get_dst_rect(src_rect.size(), size);
        ctx.draw_image_area(
            self.image_cached.as_ref().unwrap(),
            src_rect,
            dst_rect,
            InterpolationMode::Bilinear,
        );
    }
}