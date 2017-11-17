//! An example of opening an image.
extern crate image;
extern crate clap;
use clap::{App, Arg};


use std::env;
use std::fs::File;
use std::path::Path;

use image::{
    GenericImage,
    ImageBuffer,
    GrayImage,
    Pixel
};


fn create_mask(width: u32, height: u32, x_rect: u32, y_rect: u32, width_rect: u32, height_rect: u32) -> GrayImage {
    let img_created = ImageBuffer::from_fn(width, height, |x, y| {
        if x >= x_rect && x <= x_rect + width_rect && y >= y_rect && y <= y_rect + height_rect {
            image::Luma([0u8])
        } else {
            image::Luma([255u8])
        }
    });
    img_created
}

fn blended_image(width: u32, height: u32, original: &image::DynamicImage, mask: GrayImage) -> image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> {
    let blended_image = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel_image = original.get_pixel(x, y);
        let pixel_mask = mask.get_pixel(x, y);
        image::Rgba([pixel_image.data[0], pixel_image.data[1], pixel_image.data[2], 255 - pixel_mask.data[0]])
    });
    blended_image
}

fn mix_from_blurred_and_blended_image(width: u32, height: u32, blurred_image: image::DynamicImage, blended_image: image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> ) -> image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>> {
    let mut final_image_without_saturation_buff = image::ImageBuffer::new(width, height);
    for(x, y, pixel) in final_image_without_saturation_buff.enumerate_pixels_mut() {
        let pixel_target = blended_image.get_pixel(x, y);
        let mut pixel_source = blurred_image.get_pixel(x, y);
        pixel_source.blend(&pixel_target);
        *pixel = image::Rgba(pixel_source.data)
    }
    final_image_without_saturation_buff
}

fn tilt_shift_algorithm(original_image :image::DynamicImage, y_point_of_interest :u32, height_point_of_interest :u32, blur :f32, contrast :f32) -> image::DynamicImage {
    let (width, height) = original_image.dimensions();
    let mask = create_mask(width, height, 0, y_point_of_interest, width, height_point_of_interest);
    let blended_image = blended_image(width, height, &original_image, mask);

    let filtered_blurred = original_image.blur(blur);
    let final_image_without_saturation_buff = mix_from_blurred_and_blended_image(width, height, filtered_blurred, blended_image);

    return image::ImageRgba8(final_image_without_saturation_buff).adjust_contrast(contrast);
}

fn main() {
    let matches = App::new("Tilt Shift")
        .about("Compute the tilt-shift")
        .version("1.0")
        .author("Guillaume Gomez. <guillaume.gomez846@gmail.com>")
        .arg(Arg::with_name("filename")
                    .help("filename of the source image")
                    .short("f")
                    .long("filename")
                    .takes_value(true)
                    .required(true)
                    )
        .arg(Arg::with_name("blur_level")
                    .help("the amont of blur in the image")
                    .short("b")
                    .long("blur_level")
                    .takes_value(true)
                    .required(true))
        .arg(Arg::with_name("contrast_level")
                    .help("the level of contrast use in the image")
                    .short("c")
                    .long("contrast_level")
                    .takes_value(true)
                    .required(true))
        .arg(Arg::with_name("output_file_name")
                    .help("filename of the target image")
                    .short("o")
                    .long("output_file_name")
                    .takes_value(true)
                    .default_value("result.png"))
        .arg(Arg::with_name("yPointOfInterest")
                    .help("start of the focus zone")
                    .short("y")
                    .takes_value(true)
                    .long("yOrigin"))
        .arg(Arg::with_name("heightPointOfInterest")
                    .help("height of the focus zone")
                    .short("h")
                    .takes_value(true)
                    .long("height"))
        .get_matches();

    let file = matches.value_of("filename").unwrap();
    let blur = matches.value_of("blur_level").unwrap().parse::<f32>().unwrap();
    let contrast =  matches.value_of("contrast_level").unwrap().parse::<f32>().unwrap();

    let img = image::open(&Path::new(&file)).unwrap();
    let (_width, height) = img.dimensions();

    let output_file = matches.value_of("output_file_name").unwrap();


    let y_point_of_interest = if matches.is_present("yPointOfInterest") {
        matches.value_of("yPointOfInterest").unwrap().parse::<u32>().unwrap()
    } else {
        height / 3
    };

    let height_point_of_interest = if matches.is_present("heightPointOfInterest") {
        matches.value_of("heightPointOfInterest").unwrap().parse::<u32>().unwrap()
    } else {
        height / 3
    };

    let final_image = tilt_shift_algorithm(img, y_point_of_interest, height_point_of_interest, blur, contrast);
    let path_final_result = &Path::new(output_file);
    let fout_final = &mut File::create(path_final_result).unwrap();
    final_image.save(fout_final, image::PNG).unwrap();
}