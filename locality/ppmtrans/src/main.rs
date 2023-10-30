use csc411_image::Read;
use csc411_image::RgbImage;
use csc411_image::Write;
use csc411_image::Rgb;
use std::io;
use array2::Array2;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input = input.trim();

    let img = RgbImage::read(Some(trimmed_input)).unwrap();

    let init_img = Array2::new_array(img.pixels.clone(), img.width as usize, img.height as usize);

    let rotated_img = rotate_180(&init_img);

    let rotated_image = RgbImage {
        width: rotated_img.width as u32,
        height: rotated_img.height as u32,
        denominator: img.denominator,
        pixels: rotated_img.vec_of_val,
    };

    rotated_image.write(Some("output.ppm")).unwrap();
}

fn rotate_colmajor_90(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_data = Vec::new();

    for (row, col, pixel) in input_image.iter_col_major(){
        let pixel = input_image.get_element(input_image.height - row - 1, col);
        rotated_data.push(Rgb {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        });
    }

    Array2::new_array(rotated_data, input_image.height, input_image.width)
}


fn rotate_rowmajor_90(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_data = Vec::new();

    for (row, col, pixel) in input_image.iter_row_major(){
        let pixel = input_image.get_element(input_image.height - row - 1, col);
        rotated_data.push(Rgb {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        });
    }

    Array2::new_array(rotated_data, input_image.height, input_image.width)
}

fn rotate_rowmajor_180(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    let first_rotation = rotate_rowmajor_90(input_image);
    rotate_rowmajor_90(&first_rotation)
}