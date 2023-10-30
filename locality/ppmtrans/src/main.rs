use csc411_image::{Read, RgbImage, Rgb};
use std::io;
use array2::Array2;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed_input = input.trim();

    let img = RgbImage::read(Some(trimmed_input)).unwrap();

    let mut v = Vec::new();

    for elements in &img.pixels {
        v.push((elements.red as usize, elements.green as usize, elements.blue as usize));
    }

    let init_img = Array2::new(img.width as usize, img.height as usize, img.pixels);

    // Calculate the dimensions for the rotated image
    let rotated_width = init_img.height();
    let rotated_height = init_img.width();

    // Create a new vector for the rotated image
    let mut updated_vec: Vec<Rgb> = vec![Rgb { red: 0, green: 0, blue: 0 }; rotated_width * rotated_height];
}