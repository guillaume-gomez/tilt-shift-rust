//! An example of opening an image.
extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::{
    GenericImage,
    FilterType,
    ImageBuffer,
    Pixel,
    Rgb
};


// wip shoud understand why rust in soooooo complex about return function
fn create_mask(width: u32, height: u32, x_rect: u32, y_rect: u32, width_rect: u32, height_rect: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let img_created = ImageBuffer::from_fn(width, height, |x, y| {

        if x >= x_rect && x <= x_rect + width_rect && y >= y_rect && y <= y_rect + height_rect {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });
    img_created;
}

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

    let img_created = create_mask(400,400, 5, 20, 30, 50);
    let path = &Path::new("temp.png");
    //Write the contents of this image to the Writer in PNG format.
}