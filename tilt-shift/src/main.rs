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
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        panic!("Missing parameters, example : filename blur_level contrast_level [output_file_name] [yPointOfInterest] [heightPointOfInterest]");
    };

    let file = &args[1];
    let blur = args[2].parse::<f32>().unwrap();
    let contrast = args[3].parse::<f32>().unwrap();

    println!("Filename '{}' with the blur level '{}' and the contrast '{}'", file, blur, contrast);
    let img = image::open(&Path::new(&file)).unwrap();
    let (_width, height) = img.dimensions();

    println!("{:?}",&args );

    let output_file = if args.len() >= 5 {
        &args[4]
    } else {
        "result.png"
    };

    let y_point_of_interest = if args.len() >= 6 {
        args[5].parse::<u32>().unwrap()
    } else {
        height / 3
    };

    let height_point_of_interest = if args.len() >= 7 {
        args[6].parse::<u32>().unwrap()
    } else {
        height / 3
    };
    let final_image = tilt_shift_algorithm(img, y_point_of_interest, height_point_of_interest, blur, contrast);
    let path_final_result = &Path::new(output_file);
    let fout_final = &mut File::create(path_final_result).unwrap();
    final_image.save(fout_final, image::PNG).unwrap();
}