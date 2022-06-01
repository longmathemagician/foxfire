use std::sync::mpsc;
use std::ops::*;
use image::{DynamicImage, ImageBuffer};

use druid::piet::{ImageFormat, InterpolationMode, PietImage};
use druid::widget::prelude::*;
use druid::{Color, Rect};


#[derive(Debug, Copy, Clone)]
pub struct Position {
    x: f64,
    y: f64,
}
impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }
}
impl AddAssign for Position {
    fn add_assign(&mut self, other: Position) {
        self.x += other.x;
        self.y += other.y;
    }
}
impl SubAssign for Position {
    fn sub_assign(&mut self, rhs: Position) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub struct ImageTransformation {
    zoom_factor: f64,
    zoom_position: Position,
    drag_position: Position,
}

#[derive(Debug)]
struct ZoomEvent {
    delta: f64, // The distance reported by the scroll event
    position: Position, // The screen-space point of the scroll event
}
impl ZoomEvent {
    fn new(delta: f64, position: Position) -> Self {
        ZoomEvent { delta, position }
    }
}
#[derive(Debug)]
struct DragEvent {
    start_pos: Position,
    delta_pos: Position,
    finished: bool,
}
impl DragEvent {
    fn new(start_pos: Position, finished: bool,) -> Self {
        let delta_pos = Position::new(0., 0.);
        DragEvent {
            start_pos,
            delta_pos,
            finished,
        }
    }
    fn set_delta(&mut self, current_pos: Position) {
        self.delta_pos.x = current_pos.x - self.start_pos.x;
        self.delta_pos.y = current_pos.y - self.start_pos.y;
    }
}
#[derive(Debug)]
enum MouseEvent {
    Zoom(ZoomEvent),
    Drag(DragEvent),
}

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
            transform: ImageTransformation {
                zoom_factor: 0., 
                zoom_position: Position::new(0., 0.), 
                drag_position: Position::new(0., 0.),
            }
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
    fn get_src_rect(image: Size) -> Rect {
        Rect::new(0.,0.,image.width,image.height,)
    }
    fn get_paint_rect(manual_zoom_factor: f64, _zoom_position: &Position, image_center: &Position, container_width: f64, container_height: f64, image_width: f64, image_height: f64) -> Rect {
        let container_aspect_ratio = container_width/container_height;
        let image_aspect_ratio = image_width/image_height;
        let mut default_zoom = 1.;

        if image_width > container_width || image_height > container_height {
            if container_aspect_ratio > image_aspect_ratio {
                // the container is wider than the image
                let z1 = container_width/image_width;
                let z2 = container_height/image_height;
                default_zoom = if z1>z2 {z2} else {z1};
            } else {
                // the container is taller than the image
                let z1 = container_width/image_width;
                let z2 = container_height/image_height;
                default_zoom = if z1<z2 {z1} else {z2};
            }
        }


        let image_width_scaled = (image_height*(1.+manual_zoom_factor)*default_zoom)*image_aspect_ratio;
        let image_height_scaled = (image_width*(1.+manual_zoom_factor)*default_zoom)/image_aspect_ratio;

        if _zoom_position.x != 0. || _zoom_position.y != 0. {
            println!("Screen space zoom position: ({}, {})", _zoom_position.x, _zoom_position.y);

            let z_target_i_x = ImageView::map_f64(
                _zoom_position.x,
                0.,
                container_width,
                0.,
                image_width_scaled,
            );

            let z_target_i_y = ImageView::map_f64(
                _zoom_position.y,
                0.,
                container_height,
                0.,
                image_height_scaled,
            );

            println!("Partially mapped image space zoom position: ({}, {})", z_target_i_x, z_target_i_y);
        }

        Rect::new(
            (container_width/2.+image_center.x) - (image_width_scaled)/2.,
            (container_height/2.+image_center.y) - (image_height_scaled)/2.,
            (container_width/2.+image_center.x) + (image_width_scaled)/2.,
            (container_height/2.+image_center.y) + (image_height_scaled)/2.,
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
                let new_drag_event = DragEvent::new(mouse_pos, false);
                self.event_queue = Some(MouseEvent::Drag(new_drag_event));
            }
            _ctx.request_update();
        }
        else if let Event::MouseMove(mouse_event) = _event {
            if let Some(MouseEvent::Drag(drag_event)) = &mut self.event_queue {
                if !drag_event.finished {
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
                    drag_event.finished = true;
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
                drag_event.finished = true;
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
        let mut tmp_drag_pos = self.transform.drag_position;

        if let Some(MouseEvent::Drag(drag_event)) = &mut self.event_queue {
            tmp_drag_pos += drag_event.delta_pos;

            if drag_event.finished {
                self.transform.drag_position = tmp_drag_pos;
                self.event_queue = None;
            }
        }
        else if let Some(MouseEvent::Zoom(zoom_event)) = &self.event_queue {
            self.transform.zoom_factor += -zoom_event.delta/1000.;
            self.transform.zoom_factor =
                if self.transform.zoom_factor > 10. { 10. }
                else if self.transform.zoom_factor < -0.9 { -0.9 }
                else { self.transform.zoom_factor };
            self.transform.zoom_position = zoom_event.position;

            self.event_queue = None;
        }

        let src_rect = ImageView::get_src_rect(self.image_size);
        let dst_rect = Rect::new(0.0, 0.0, size.width, size.height,);

        ctx.draw_image_area(
            self.image_cached.as_ref().unwrap(),
            src_rect,
            dst_rect,
            InterpolationMode::Bilinear,
        );
    }
}