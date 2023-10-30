use csc411_image::Read;
use csc411_image::RgbImage;
use csc411_image::Write;
use csc411_image::Rgb;
use clap::Parser;
use std::io;
use array2::Array2;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {
    // Flip
    #[clap(long = "flip", required = false)]
    flip: Option<String>,
    //  Transpose
    #[clap(long = "transpose")]
    transpose: bool,
    // Rotation
    #[clap(short = 'r', long = "rotate")]
    rotate: Option<u32>,
    // Col Major Type
    #[clap(long = "col-major")]
    col_major: bool,
    // Row Major Type
    #[clap(long = "row-major")]
    row_major: bool,
    // File Input
    #[clap()]
    file_given: Option<String>,

}

fn main() {
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Failed to read line");

    // let trimmed_input = input.trim();

    let args = Args::parse();
    let rotate = args.rotate;
    let col_major = args.col_major;
    let row_major = args.row_major;
    let flip = args.flip;
    let transpose = args.transpose;

    let input = args.file_given;
    let img = RgbImage::read(input.as_deref()).unwrap();

    //let img = RgbImage::read(Some(trimmed_input)).unwrap();

    let init_img = Array2::new_array(img.pixels.clone(), img.width as usize, img.height as usize);

    if row_major {
        match rotate {
            Some(90) => {
                let rotated_img = rotate_rowmajor_90(&init_img);
    
                let rotated_image = RgbImage {
                    width: rotated_img.width as u32,
                    height: rotated_img.height as u32,
                    denominator: img.denominator,
                    pixels: rotated_img.vec_of_val,
                };
                rotated_image.write(Some("output.ppm")).unwrap();
            }
            Some(180) => {
                let rotated_img = rotate_rowmajor_180(&init_img);
    
                let rotated_image = RgbImage {
                    width: rotated_img.width as u32,
                    height: rotated_img.height as u32,
                    denominator: img.denominator,
                    pixels: rotated_img.vec_of_val,
                };
                rotated_image.write(Some("output.ppm")).unwrap();
            }
            // Add other rotation cases here
            _ => {
                // Handle other rotation angles if needed
            }
        }
    }    

    // let rotated_img = rotate_colmajor_180(&init_img);

    // let rotated_image = RgbImage {
    //     width: rotated_img.width as u32,
    //     height: rotated_img.height as u32,
    //     denominator: img.denominator,
    //     pixels: rotated_img.vec_of_val,
    // };

    // rotated_image.write(Some("output.ppm")).unwrap();
}

fn rotate_colmajor_90(input_image: &Array2<Rgb>) -> Array2<Rgb> {
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

fn rotate_colmajor_180(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    let first_rotation = rotate_colmajor_90(input_image);
    rotate_colmajor_90(&first_rotation)
}

fn rotate_rowmajor_180(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    let first_rotation = rotate_rowmajor_90(input_image);
    rotate_rowmajor_90(&first_rotation)
}