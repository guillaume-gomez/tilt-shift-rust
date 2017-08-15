//! An example of opening an image.
extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::{
    GenericImage,
    FilterType
};

fn main() {
    let file = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a file")
    };

    // Use the open function to load an image from a PAth.
    // ```open``` returns a dynamic image.
    let img = image::open(&Path::new(&file)).unwrap();

    // The dimensions method returns the images width and height
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's ColorType
    println!("{:?}", img.color());

    let filtered = img.fliph().resize(300,300, FilterType::Nearest);
    let fout = &mut File::create(&Path::new(&format!("{}.png", file))).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    filtered.save(fout, image::PNG).unwrap();
}