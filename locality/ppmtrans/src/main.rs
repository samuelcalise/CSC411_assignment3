#[warn(unused_imports)]
use csc411_image::{Read, RgbImage, Rgb};
#[warn(unused_imports)]
use array2::Array2;
use clap::Parser;

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
    println!("here");
}