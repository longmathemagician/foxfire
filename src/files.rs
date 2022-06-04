use std::{env, thread};
use std::ops::{Deref, DerefMut};
use std::path::{Path, PathBuf};
use std::sync::{Arc, mpsc};
use std::sync::mpsc::Receiver;
use image::DynamicImage;

pub struct AsyncImageLoader {
    path: String,
    image: Option<Receiver<DynamicImage>>
}

impl AsyncImageLoader {
    pub fn new() -> Self {
        Self {
            path: String::new(),
            image: None,
        }
    }
    pub fn new_from_string(path: String) -> Self {
        Self {
            path,
            image: None,
        }
    }
    pub fn load_image(&mut self) {
        // Spawn a thread that loads the image, pass back a receiver to retrieve it when it's done
        let (tx_data, rx_data) = mpsc::channel();
        self.image = Some(rx_data);
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
        self.image.take()
    }
    pub fn has_receiver(&self) -> bool {
        match &self.image {
            Some(_) => true,
            _ => false,
        }
    }
}
