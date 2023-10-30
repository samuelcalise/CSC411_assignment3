use csc411_image::Read;
use csc411_image::RgbImage;
use csc411_image::Write;
use clap::Parser;
use array2::Array2;
mod thething;
use thething::{rotate_colmajor_90, rotate_rowmajor_90, rotate_colmajor_180, rotate_rowmajor_180};
use std::process;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]

struct Args {

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
    file_name: Option<String>,

}

//cargo run -- --rotate 180 --col-major ./f_original.ppm > here.ppm
fn main() {

    let args = Args::parse();
    let rotate = args.rotate;
    let col_major = args.col_major;
    let row_major = args.row_major;


    let input = args.file_name;
    let img = RgbImage::read(input.as_deref()).unwrap();

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
                rotated_image.write(None).unwrap();
            }
            Some(180) => {
                let rotated_img = rotate_rowmajor_180(&init_img);
    
                let rotated_image = RgbImage {
                    width: rotated_img.width as u32,
                    height: rotated_img.height as u32,
                    denominator: img.denominator,
                    pixels: rotated_img.vec_of_val,
                };
                rotated_image.write(None).unwrap();
            }
            // Add other rotation cases here
            _ => {
                eprintln!("Totally gahbage");
                process::exit(1);
            }
        }
    }
    else if col_major{
        match rotate{
            Some(90) => {
                let rotated_img = rotate_colmajor_90(&init_img);
    
                let rotated_image = RgbImage {
                    width: rotated_img.width as u32,
                    height: rotated_img.height as u32,
                    denominator: img.denominator,
                    pixels: rotated_img.vec_of_val,
                };
                rotated_image.write(None).unwrap();
            }
            Some(180) => {
                let rotated_img = rotate_colmajor_180(&init_img);
    
                let rotated_image = RgbImage {
                    width: rotated_img.width as u32,
                    height: rotated_img.height as u32,
                    denominator: img.denominator,
                    pixels: rotated_img.vec_of_val,
                };
                rotated_image.write(None).unwrap();
            }
            // Add other rotation cases here
            _ => {
                eprintln!("Totally gahbage");
                process::exit(1);
            }
        }
    }
}