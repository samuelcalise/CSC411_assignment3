/*
    Required used/imports
*/
use csc411_image::Read;
use csc411_image::RgbImage;
use csc411_image::Write;
use clap::Parser;
use array2::Array2;
mod thething;
use thething::{rotate_colmajor_90, rotate_rowmajor_90, rotate_colmajor_180, rotate_rowmajor_180};
use std::process;
use std::time::Instant;

/*
    Required Synatx for Clap
*/
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
/*
    Main Function:
        Intended to use clap command line structure to call the expended function to
        rotate an ppm image to a new rotated ppm image.
*/
fn main() {

    // Doing clap parsing
    let args = Args::parse();
    let rotate = args.rotate;
    let col_major = args.col_major;
    let row_major = args.row_major;
    let input = args.file_name;

    //Finding the image within the directory level of doing 'cargo run ...'
    let img = RgbImage::read(input.as_deref()).unwrap();

    // Creating an initial image for the RGB image
    let init_img = Array2::new_array(img.pixels.clone(), img.width as usize, img.height as usize);

    //When the clap args finds a 'row-major' within the command line ==> True
    if row_major {
        //When clap args finds either 90 or 180, the match function will handle the certain matching case
        //and rotate the img according 
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
            //The case where the another rotate degree is not supported
            _ => {
                eprintln!("Totally gahbage");
                process::exit(1);
            }
        }
    }
    //When the clap args finds a 'col-major' within the command line ==> True
    else if col_major{
        //When clap args finds either 90 or 180, the match function will handle the certain matching case
        //and rotate the img according
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
            //The case where the another rotate degree is not supported
            _ => {
                eprintln!("Totally gahbage");
                process::exit(1);
            }
        }
    }
}