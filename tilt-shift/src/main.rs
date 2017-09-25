//! An example of opening an image.
extern crate image;

use std::env;
use std::fs::File;
use std::path::Path;

use image::{
    GenericImage,
    FilterType,
    ImageBuffer,
    GrayImage,
    Pixel
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
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("Missing parameters, example : filename blur_level contrast_level");
    };

    let file = &args[1];
    let blur = args[2].parse::<f32>().unwrap();
    let contrast = args[3].parse::<f32>().unwrap();

    println!("Filename '{}' with the blur level '{}' and the contrast '{}'", file, blur, contrast);

    let output_file = if args.len() == 5 {
        &args[4]
    } else {
        "result.png"
    };

    let img = image::open(&Path::new(&file)).unwrap();
    let (width, height) = img.dimensions();

    // resize image
    let filtered = img.resize_exact(width, height, FilterType::Nearest);

    let mask = create_mask(width, height, 0, 150, height, 350);

    let blend_image = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel_image = filtered.get_pixel(x, y);
        let pixel_mask = mask.get_pixel(x, y);
        image::Rgba([pixel_image.data[0], pixel_image.data[1], pixel_image.data[2], 255 - pixel_mask.data[0]])
    });

    let filtered_blurred = filtered.blur(blur);
    let mut final_image_without_saturation_buff = image::ImageBuffer::new(width, height);
    for(x, y, pixel) in final_image_without_saturation_buff.enumerate_pixels_mut() {
        let pixel_target = blend_image.get_pixel(x, y);
        let mut pixel_source = filtered_blurred.get_pixel(x, y);
        pixel_source.blend(&pixel_target);
        *pixel = image::Rgba(pixel_source.data)
    }

    let final_image = image::ImageRgba8(final_image_without_saturation_buff).adjust_contrast(contrast);
    let path_final_result = &Path::new(output_file);
    let fout_final = &mut File::create(path_final_result).unwrap();
    final_image.save(fout_final, image::PNG).unwrap();
}