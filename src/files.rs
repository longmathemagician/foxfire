use image::DynamicImage;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;

pub struct AsyncImageLoader {
    path: String,
    image_receiver: Option<Receiver<DynamicImage>>,
    image: Option<DynamicImage>,
}

impl AsyncImageLoader {
    pub fn from_str(new_path: &str) -> Self {
        Self {
            path: new_path.to_string(),
            image_receiver: None,
            image: None,
        }
    }
    pub fn load_image(&mut self) {
        // Spawn a thread that loads the image, pass back a receiver to retrieve it when it's done
        let (tx_data, rx_data) = mpsc::channel();
        self.image_receiver = Some(rx_data);
        let image_path = PathBuf::from(&self.path);
        // println!("Trying to load from: {:#?}", image_path);
        thread::spawn(move || {
            let img = image::open(&image_path).unwrap();
            // println!("Open? image: {:#?}", img);
            match tx_data.send(img) {
                Ok(_) => println!("Loaded image from: {:#?}", image_path),
                Err(e) => println!("Error loading image: {:#?}", e),
            };
        });
    }
}

// #[derive(Clone, Data)]
pub struct NewImageContainer {
    pub path: String,
    pub image: DynamicImage,
}

impl NewImageContainer {
    pub fn from_string_and_dynamicimage(path: String, image: DynamicImage) -> Self {
        Self { path, image }
    }
}
