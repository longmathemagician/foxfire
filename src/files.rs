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
    pub fn new() -> Self {
        Self {
            path: String::new(),
            image_receiver: None,
            image: None,
        }
    }
    pub fn new_from_string(new_path: &str) -> Self {
        Self {
            path: new_path.to_string(),
            image_receiver: None,
            image: None,
        }
    }
    pub fn new_from_bytes(image: DynamicImage) -> Self {
        Self {
            path: String::new(),
            image_receiver: None,
            image: Some(image),
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
    pub fn take_image_receiver(&mut self) -> Option<Receiver<DynamicImage>> {
        self.image_receiver.take()
    }
    pub fn has_receiver(&self) -> bool {
        matches!(&self.image_receiver, Some(_))
    }
    pub fn has_image(&self) -> bool {
        matches!(&self.image, Some(_))
    }
    pub fn take_image(&mut self) -> Option<DynamicImage> {
        self.image.take()
    }
}
