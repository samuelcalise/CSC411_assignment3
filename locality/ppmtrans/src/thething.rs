use array2::Array2;
use csc411_image::Read;
use csc411_image::RgbImage;
use csc411_image::Write;
use csc411_image::Rgb;
use clap::Parser;
use std::io;

pub fn rotate_colmajor_90(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_data = Vec::new();

    for (col, row, pixel) in input_image.iter_col_major(){
        let pixel = input_image.get_element(input_image.height - row - 1, col);
        rotated_data.push(Rgb {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        });
    }

    Array2::new_array(rotated_data, input_image.height, input_image.width)
}


pub fn rotate_rowmajor_90(input_image: &Array2<Rgb>) -> Array2<Rgb> {
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


pub fn rotate_colmajor_180(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    let mut rotated_data = Vec::new();

    for (col, row, pixel) in input_image.iter_row_major(){
        let pixel = input_image.get_element( input_image.height - row - 1, input_image.width - col - 1,);
        rotated_data.push(Rgb {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        });
    }

    Array2::new_array(rotated_data, input_image.height, input_image.width)
}

pub fn rotate_rowmajor_180(input_image: &Array2<Rgb>) -> Array2<Rgb> {

    let mut rotated_data = Vec::new();

    for (row, col, pixel) in input_image.iter_row_major(){
        let pixel = input_image.get_element( input_image.height - col - 1, input_image.width - row - 1,);
        rotated_data.push(Rgb {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        });
    }

    Array2::new_array(rotated_data, input_image.height, input_image.width)
}