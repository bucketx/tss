use std::env;

use image::imageops::FilterType;
use image::io::Reader as ImageReader;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        for arg in &args[1..] {
            let input_image_reader = match ImageReader::open(arg) {
                Ok(reader) => reader,
                Err(_) => {
                    println!("ERROR: Failed to open image '{}' . . . SKIPPED", arg);
                    continue;
                }
            };

            let input_image = match input_image_reader.decode() {
                Ok(image) => image,
                Err(_) => {
                    println!("ERROR: Failed to decode image '{}' . . . SKIPPED", arg);
                    continue;
                }
            };

            let output_image_filename = format!("{}{}", arg, ".png");
            match input_image
                .resize(512, 512, FilterType::Triangle)
                .save(&output_image_filename)
            {
                Ok(_) => {}
                Err(_) => {
                    println!(
                        "ERROR: Failed to write image to '{}' file",
                        &output_image_filename
                    );
                    continue;
                }
            }
        }
    } else {
        println!("Usage: {} <image> ...", &args[0])
    }
}
