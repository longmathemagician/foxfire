use std::{env};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbImage};
use std::sync::mpsc;
use std::thread;

use druid::piet::{ImageFormat, InterpolationMode, PietImage};
use druid::widget::prelude::*;
use druid::{AppLauncher, Color, LocalizedString, Rect, WindowDesc};

struct ImageView {
	image_src: std::sync::mpsc::Receiver<image::DynamicImage>, 
	image_data: DynamicImage, 
	image_width: usize, 
	image_height: usize, 
	aspect_ratio: f64,
	image_cached: Option<PietImage>,
	panning: bool,
}

impl Widget<String> for ImageView {
	fn paint(&mut self, ctx: &mut PaintCtx, data: &String, env: &Env) {
		let size = ctx.size();
		let rect = size.to_rect();
		ctx.fill(rect, &Color::WHITE);

		if let None = self.image_cached {
			let tmp = ctx.make_image(self.image_width, self.image_height, &self.image_data.as_bytes(), ImageFormat::Rgb);
			self.image_cached = Some(tmp.unwrap());
		}

		let mut cxt_size = size.to_vec2();
		ctx.draw_image(&self.image_cached.as_ref().unwrap(), Rect::new(
			0., 
			0., 
			cxt_size.x,
			cxt_size.y
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
			println!("Zoomed {:#?} at {}", mouse_event.wheel_delta.y, mouse_event.window_pos);
		}
		else if let druid::Event::MouseDown(mouse_event) = _event {
			self.panning = true;
		}
		else if let druid::Event::MouseMove(mouse_event) = _event {
			if self.panning {
				println!("Mouse drag event at {}", mouse_event.window_pos);
			}
		}
		else if let druid::Event::MouseUp(mouse_event) = _event {
			self.panning = false;
		}
	}
	fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &String, _env: &Env,) {
		if let LifeCycle::WidgetAdded = _event {
			// Receive the image from the thread
			let received_image_handle = self.image_src.try_recv(); // blocks, fix this next
			self.image_data = match received_image_handle {
				Ok(image) => image,
				Err(_) => DynamicImage::ImageRgb8(ImageBuffer::new(1, 1)),
			};
			self.image_width = self.image_data.width().try_into().unwrap();
			self.image_height = self.image_data.height().try_into().unwrap();
			self.aspect_ratio = self.image_width as f64 / self.image_height as f64;
		}
	}
	fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {
		
	}
}

fn main() {
	// Get command line arguments
	let args: Vec<String> = env::args().collect();

	// Set the name of the file to load from the command line args, if they exist
	let file_name = if args.len()>1 {args[1].clone()} else {String::from("/home/steve/Projects/foxfire/jazz.bmp")};

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
			image_src: rx_data, 
			image_data: DynamicImage::ImageRgb8(ImageBuffer::new(1, 1)), 
			image_width: 1, 
			image_height: 1,
			aspect_ratio: 1.,
			image_cached: None,
			panning: false,
		});

	AppLauncher::with_window(window)
		.log_to_console()
		.launch("---TEST---".to_string())
		.expect("launch failed");
}
