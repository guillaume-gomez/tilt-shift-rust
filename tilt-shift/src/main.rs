//! An example of opening an image.
extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::{
    GenericImage,
    FilterType,
    ImageBuffer,
    GrayImage
};


fn create_mask (width: u32, height: u32, x_rect: u32, y_rect: u32, width_rect: u32, height_rect: u32) -> GrayImage {
    let img_created = ImageBuffer::from_fn(width, height, |x, y| {
        if x >= x_rect && x <= x_rect + width_rect && y >= y_rect && y <= y_rect + height_rect {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });
    img_created
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
    println!("start filtered blured");
    let filtered = img.fliph().resize_exact(600, 600, FilterType::Nearest);
    let fout = &mut File::create(&Path::new(&format!("{}.png", file))).unwrap();

    // Write the contents of this image to the Writer in PNG format.
    filtered.save(fout, image::PNG).unwrap();
    println!("end filtered blured");
    println!("start mask ");
    let mask = create_mask(600, 600, 0, 200, 600, 200);
    let path = &Path::new("mask.png");
    mask.save(path).unwrap();
    println!("end mask ");
    let mask_blurred = image::open(path).unwrap().blur(10.0);
    //let path_2 = &Path::new("mask_blurred.png");
    //let fout2 = &mut File::create(path_2).unwrap();
    //mask_blurred.save(fout2, image::PNG).unwrap();
    println!("start blend_image ");
    let blend_image = ImageBuffer::from_fn(600, 600, |x, y| {
        let pixel_image = filtered.get_pixel(x, y);
        let pixel_mask = mask_blurred.get_pixel(x, y);
        image::Rgba([pixel_image.data[0], pixel_image.data[1], pixel_image.data[2], 255 - pixel_mask.data[0]])
    });
    println!("end blend_image");
    let path_3 = &Path::new("image_mask_blurred.png");
    blend_image.save(path_3).unwrap();

    let filtered_blurred = filtered.blur(2.5);
    let final_image_without_saturation = ImageBuffer::from_fn(600, 600, |x, y| {
        let pixel_target = blend_image.get_pixel(x, y);
        let pixel_source = filtered_blurred.get_pixel(x, y);
        image::Rgba([pixel_mask.data[0], pixel_mask.data[1], pixel_mask.data[2], pixel_blend.data[0]])
    });
    final_image_without_saturation.save(&Path::new("result.png")).unwrap();


}