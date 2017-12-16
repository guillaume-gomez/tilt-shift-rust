//! An example of opening an image.
extern crate image;
extern crate clap;
use clap::{App, Arg};

use std::fs;

use std::path::Path;

use image::{
    GenericImage
};

mod tilt_shift_module;

fn create_single_image(matches: clap::ArgMatches) {
    let file = matches.value_of("filename").unwrap();
    let blur = matches.value_of("blur_level").unwrap().parse::<f32>().unwrap();
    let saturation =  matches.value_of("saturation_level").unwrap().parse::<f32>().unwrap();

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

    tilt_shift_module::create_image(file, output_file, blur, saturation, y_point_of_interest, height_point_of_interest);
}

fn create_several_images(matches: clap::ArgMatches) {
    let file = matches.value_of("filename").unwrap();
    
    let img = image::open(&Path::new(&file)).unwrap();
    let (_width, height) = img.dimensions();

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
    let output_file_folder = if matches.is_present("outputFolderName") {
        matches.value_of("outputFolderName").unwrap()
    } else {
        "output"
    };
    fs::create_dir(output_file_folder);
    

    let range_blur = parse_params(matches.value_of("blur_level"));
    let range_saturation = parse_params(matches.value_of("saturation_level"));

    for current_blur in range_blur[0]..range_blur[1] {
        for current_saturation in range_saturation[0]..range_saturation[1] {
            let output_file = format!("{}/{}_{}_{}", output_file_folder, current_blur , current_saturation, matches.value_of("output_file_name").unwrap());
            tilt_shift_module::create_image(file, &output_file, (current_blur as f32), (current_saturation as f32), y_point_of_interest, height_point_of_interest);
            println!("image '{}' with blur_level = '{}' and saturation_level = '{}' generated", output_file, current_blur, current_saturation);
        }
    }
}

fn parse_params(param: Option<&str>) -> Vec<i32> {
  let has_loop = param.unwrap().contains("..");
  if has_loop  {
    let range: Vec<&str> = param.unwrap().split("..").collect();
    let min = range[0].parse::<i32>().unwrap();
    let max = range[1].parse::<i32>().unwrap();
     vec![min, max + 1]
  } else {
    let val = param.unwrap().parse::<i32>().unwrap();
    return vec![val, val + 1];
  }
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
        .arg(Arg::with_name("saturation_level")
                    .help("the level of saturation use in the image")
                    .short("s")
                    .long("saturation_level")
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
        .arg(Arg::with_name("outputFolderName")
                    .help("output folder name")
                    .short("w")
                    .takes_value(true)
                    .long("folder_name"))
        
        .get_matches();

    let has_loop = matches.value_of("blur_level").unwrap().contains("..");
    if has_loop {
        create_several_images(matches)
    } else {
        create_single_image(matches);
    }

}