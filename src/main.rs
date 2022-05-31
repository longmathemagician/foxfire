#![allow(clippy::collapsible_else_if)]
use std::{env};
use std::sync::{Arc, mpsc};
use std::ops::*;
use image::{DynamicImage, ImageBuffer};
use std::thread;

use druid::piet::{ImageFormat, InterpolationMode, PietImage};
use druid::widget::prelude::*;
use druid::{AppLauncher, Color, LocalizedString, Rect, WindowDesc};

#[derive(Debug, Copy, Clone)]
struct Position {
	x: f64,
	y: f64,
}
impl Position {
	fn new(x: f64, y: f64) -> Self {
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
struct ImageTransformation {
	zoom_factor: f64,
	zoom_position: Position,
	drag_position: Position,
}
#[derive(Debug)]
struct Zoom {
	delta: f64, // The distance reported by the scroll event
	position: Position, // The location of the cursor where the scroll event occured
}
impl Zoom {
	fn new(delta: f64, position: Position) -> Self {
		Zoom { delta, position }
	}
}
#[derive(Debug)]
struct Drag {
	start_pos: Position,
	delta_pos: Position,
	finished: bool,
}
impl Drag {
	fn new(start_pos: Position, finished: bool,) -> Self {
		let delta_pos = Position::new(0., 0.);
		Drag {
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
	zoom(Zoom),
	drag(Drag),
}

struct ImageView {
	image_path: Option<Arc<std::path::Path>>,
	image_src: std::sync::mpsc::Receiver<image::DynamicImage>, 
	image_data: DynamicImage, 
	image_width: usize, 
	image_height: usize, 
	aspect_ratio: f64,
	image_cached: Option<PietImage>,
	event_queue: Option<MouseEvent>,
	transform: ImageTransformation,
}
impl ImageView {
	fn mapf(value: f64, in_l: f64, in_u: f64, out_l: f64, out_u: f64, ) -> f64 {
		let result = out_l + (value - in_l)*(out_u - out_l)/(in_u - in_l);
		result
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

			let _zpix = ImageView::mapf(
				_zoom_position.x,
				0.,
				container_width,
				0.,
				image_width_scaled,
			);

			let _zpiy = ImageView::mapf(
				_zoom_position.y,
				0.,
				container_height,
				0.,
				image_height_scaled,
			);

			println!("Partially mapped image space zoom position: ({}, {})", _zpix, _zpiy);			
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

	fn paint(&mut self, ctx: &mut PaintCtx, _data: &String, _env: &Env) {
		let size = ctx.size();
		let rect = size.to_rect();
		ctx.fill(rect, &Color::WHITE);

		if self.image_cached.is_none() {
			let tmp = ctx.make_image(self.image_width, self.image_height, self.image_data.as_bytes(), ImageFormat::Rgb);
			self.image_cached = Some(tmp.unwrap());
		}
		let mut tmp_drag_pos = self.transform.drag_position;

		if let Some(MouseEvent::drag(drag_event)) = &mut self.event_queue {
			tmp_drag_pos += drag_event.delta_pos;
			
			if drag_event.finished {
				self.transform.drag_position = tmp_drag_pos;
				self.event_queue = None;
			}
		}
		else if let Some(MouseEvent::zoom(zoom_event)) = &self.event_queue {
			self.transform.zoom_factor += -zoom_event.delta/1000.;
			self.transform.zoom_factor = 
				if self.transform.zoom_factor > 10. { 10. } 
				else if self.transform.zoom_factor < -0.9 { -0.9 } 
				else { self.transform.zoom_factor };
			self.transform.zoom_position = zoom_event.position;

			self.event_queue = None;
		}

		ctx.draw_image(self.image_cached.as_ref().unwrap(), 
			ImageView::get_paint_rect(
				self.transform.zoom_factor, 
				&self.transform.zoom_position, 
				&tmp_drag_pos, 
				size.width, 
				size.height, 
				self.image_width as f64, 
				self.image_height as f64,
			), InterpolationMode::Bilinear);
	}
	fn layout(&mut self, _layout_ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &String, _env: &Env,) -> Size {
		if bc.is_width_bounded() && bc.is_height_bounded() {
			bc.max()
		} else {
			let size = Size::new(100., 100.);
			bc.constrain(size)
		}

	}
	fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut String, _env: &Env) {
		if let druid::Event::Wheel(mouse_event) = _event {
			if self.event_queue.is_none() {
				let mouse_position = Position::new(mouse_event.window_pos.x, mouse_event.window_pos.y);
				self.event_queue = Some(MouseEvent::zoom(Zoom::new(mouse_event.wheel_delta.y, mouse_position)));
			}
			_ctx.request_update();
		}
		else if let druid::Event::MouseDown(mouse_event) = _event {
			if self.event_queue.is_none() {
				let mouse_pos = Position::new(mouse_event.window_pos.x, mouse_event.window_pos.y);
				let new_drag_event = Drag::new(mouse_pos, false);
				self.event_queue = Some(MouseEvent::drag(new_drag_event));
			}
			_ctx.request_update();
		}
		else if let druid::Event::MouseMove(mouse_event) = _event {
			if let Some(MouseEvent::drag(drag_event)) = &mut self.event_queue {
				if !drag_event.finished {
					let current_pos = Position::new(mouse_event.window_pos.x, mouse_event.window_pos.y);
					drag_event.set_delta(current_pos);
					_ctx.request_update();
				}	
			}
		}
		else if let druid::Event::MouseUp(_mouse_event) = _event {
			if let Some(active_event) = &mut self.event_queue {
				if let MouseEvent::drag(drag_event) = active_event {
					drag_event.finished = true;
				}
				_ctx.request_update();
			}
		}
	}
	fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &String, _env: &Env,) {
		if let LifeCycle::WidgetAdded = _event {
			// Receive the image from the thread
			let received_image_handle = self.image_src.recv();
			self.image_data = match received_image_handle {
				Ok(image) => {image},
				Err(_) => DynamicImage::ImageRgb8(ImageBuffer::new(1, 1)),
			};
			self.image_width = self.image_data.width().try_into().unwrap();
			self.image_height = self.image_data.height().try_into().unwrap();
			self.aspect_ratio = self.image_width as f64 / self.image_height as f64;
			// if let Some(_title) = &self.image_path {
			// 	_env.set(druid::WindowHandle.title);
			// }
		}
		if let LifeCycle::FocusChanged(false) | LifeCycle::HotChanged(false)  = _event {
			if let Some(MouseEvent::drag(drag_event)) = &mut self.event_queue {
				drag_event.finished = true;
			}
		}
	}
	fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {
		_ctx.request_paint();
	}
}

fn main() {
	// Get command line arguments
	let args: Vec<String> = env::args().collect();

	// Set the name of the file to load from the command line args, if they exist
	let file_name = if args.len()>1 {args[1].clone()} else {String::from("/home/steve/1.jpg")};

	// Send the name of the file to the thread that will load the image in the background
	let (tx_name, rx_name) = mpsc::channel();
	tx_name.send(file_name).unwrap();

	// Spawn a thread, load the image, and pass it back
	let (tx_data, rx_data) = mpsc::channel();
	thread::spawn(move || {
		let name = rx_name.recv().unwrap();
		let file_path = std::path::Path::new(&name);

		let img = image::open(file_path).unwrap();
		tx_data.send(img).unwrap();
	});

	// Open a window to display the image, passing it the receiver for the image along with initial data
	let window = WindowDesc::new(
		ImageView {
			image_path: None,
			image_src: rx_data, 
			image_data: DynamicImage::ImageRgb8(ImageBuffer::new(1, 1)), 
			image_width: 1, 
			image_height: 1,
			aspect_ratio: 1.,
			image_cached: None,
			event_queue: None,
			transform: ImageTransformation {zoom_factor: 0., zoom_position: Position::new(0., 0.), drag_position: Position::new(0., 0.)}
		})
		.title(LocalizedString::new("Linux Photo Viewer"))
		.window_size((640., 480.));

	AppLauncher::with_window(window)
		.log_to_console()
		.launch("launch string".to_string())
		.expect("launch failed");
}
