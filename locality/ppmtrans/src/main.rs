use csc411_image::{Read};
use csc411_image::RgbImage;
use std::io;
use array2::Array2;
use csc411_image::Write;
use csc411_image::Rgb;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input = input.trim();

    let img = RgbImage::read(Some(trimmed_input)).unwrap();

    let init_img = Array2::new_array(img.pixels.clone(), img.width as usize, img.height as usize); // Clone the pixel data

    let rotated_img = rotate_90(&init_img);

    let rotated_image = RgbImage {
        width: img.height,
        height: img.width,
        denominator: img.denominator,
        pixels: rotated_img.vec_of_val,
    };

    rotated_image.write(Some("output.ppm"));
}

fn rotate_90(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_data = Vec::new();

    for col in 0..input_image.width {
        for row in (0..input_image.height).rev() {
            let pixel = input_image.get_element(row, col);
            rotated_data.push(Rgb {
                red: pixel.red,
                green: pixel.green,
                blue: pixel.blue,
            });
        }
    }

    Array2::new_array(rotated_data, input_image.height, input_image.width)
}