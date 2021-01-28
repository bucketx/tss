use image::imageops::FilterType;
use image::io::Reader as ImageReader;

use clap::clap_app;

fn main() {
    let tss = clap_app!(tss =>
        (version: "0.0.1")
        (author: "BucketX")
        (about: "Telegram Sticker Scaler")
        (@arg input: -i --input +required +takes_value ... "Set input images")
        (@arg output: -o --output +takes_value "Set output directory")
    )
    .get_matches();

    let input_images: Vec<&str> = tss
        .values_of("input")
        .expect("Input images are required.")
        .collect();
    let mut output_directory = String::from(tss.value_of("output").unwrap_or("").trim());

    if output_directory.len() != 0 && output_directory.ends_with("/") {
        output_directory.push('/');
    }

    for image in input_images {
        let input_image_reader = match ImageReader::open(image) {
            Ok(reader) => reader,
            Err(err) => {
                println!(
                    "ERROR: Failed to open image '{}' -> {} . . . SKIPPED",
                    image, err
                );
                continue;
            }
        };

        let input_image = match input_image_reader.decode() {
            Ok(image) => image,
            Err(err) => {
                println!(
                    "ERROR: Failed to decode image '{}' -> {} . . . SKIPPED",
                    image, err
                );
                continue;
            }
        };

        let output_image_filename = format!("{}{}{}", output_directory, image, ".png");
        match input_image
            .resize(512, 512, FilterType::Triangle)
            .save(&output_image_filename)
        {
            Ok(_) => {
                println!("INFO: Created new image '{}'", &output_image_filename);
            }
            Err(_) => {
                println!(
                    "ERROR: Failed to write image to '{}' file",
                    &output_image_filename
                );
                continue;
            }
        }
    }
}
