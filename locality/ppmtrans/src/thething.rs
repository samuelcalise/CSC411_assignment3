use array2::Array2;
use csc411_image::Rgb;
//use std::time::Instant;


/// Function: Rotate Row Major 90
///
/// The invariant in this function is getting a pixel at a certain coordinate
/// and "pushing" the pixel into an updated array2.
///
/// This function's parameter is taking the input image from the command line
/// and references it's array2 1d vector. When iterating through the vector,
/// the function will return an `array2<Rgb>` which is the expected rotated
/// image for 90 degrees clockwise rotation in row_major iteration.
pub fn rotate_rowmajor_90(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    let rotated_data = vec![Rgb{red: 0,green: 0, blue: 0};input_image.width * input_image.height];
    // let now = Instant::now();
    
    let mut array = Array2::new_array(rotated_data, input_image.height, input_image.width);

    for (col, row, _pixel) in input_image.iter_row_major(){
        // --> eprintln!("{},{}, {}",input_image.height, row, col);
        let pixel = array.get_element(input_image.height - row - 1, col);
        *pixel = _pixel.clone();
    }
    
    // let elapsed = now.elapsed();
    // eprintln!("{:.2?}", elapsed);
    
    array
}

/// Function: Rotate Row Major 180
///
/// The invariant in this function is getting a pixel at a certain coordinate
/// and "pushing" the pixel into an updated array2.
///
/// This function's parameter is taking the input image from the command line
/// and references it's array2 1d vector. When iterating through the vector,
/// the function will return an `array2<Rgb>` which is the expected rotated
/// image for 180 degrees clockwise rotation in row_major iteration.
pub fn rotate_rowmajor_180(input_image: &Array2<Rgb>) -> Array2<Rgb> {

    let rotated_data = vec![Rgb{red: 0,green: 0, blue: 0};input_image.width * input_image.height];
    // let now = Instant::now();
    
    let mut array = Array2::new_array(rotated_data, input_image.width, input_image.height);

    // (w − i − 1, h − j − 1).
    for (col, row, _pixel) in input_image.iter_row_major(){
        // --> eprintln!("{},{}, {}",input_image.height, row, col);
        let pixel = array.get_element( input_image.width - col - 1, input_image.height - row - 1);
        *pixel = _pixel.clone();
    }
    
    // let elapsed = now.elapsed();
    // eprintln!("{:.2?}", elapsed);
    
    array
}

/// Function: Rotate Column Major 90
///
/// The invariant in this function is getting a pixel at a certain coordinate
/// and "pushing" the pixel into an updated array2.
///
/// This function's parameter is taking the input image from the command line
/// and references it's array2 1d vector. When iterating through the vector,
/// the function will return an `array2<Rgb>` which is the expected rotated
/// image for 90 degrees clockwise rotation in col_major iteration.
pub fn rotate_colmajor_90(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    
    let rotated_data = vec![Rgb{red: 0,green: 0, blue: 0};input_image.width * input_image.height];
    // let now = Instant::now();
    
    let mut array = Array2::new_array(rotated_data, input_image.height, input_image.width);

    for (col, row, _pixel) in input_image.iter_col_major(){
        // --> eprintln!("{},{}, {}",input_image.height, row, col);
        let pixel = array.get_element(input_image.height - row - 1, col);
        *pixel = _pixel.clone();
    }
    
    // let elapsed = now.elapsed();
    // eprintln!("{:.2?}", elapsed);
    
    array

}

/// Function: Rotate Column Major 180
///
/// The invariant in this function is getting a pixel at a certain coordinate
/// and "pushing" the pixel into an updated array2.
///
/// This function's parameter is taking the input image from the command line
/// and references it's array2 1d vector. When iterating through the vector,
/// the function will return an `array2<Rgb>` which is the expected rotated
/// image for 180 degrees clockwise rotation in col_major iteration.
pub fn rotate_colmajor_180(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    
    let rotated_data = vec![Rgb{red: 0,green: 0, blue: 0};input_image.width * input_image.height];
    // let now = Instant::now();
    
    let mut array = Array2::new_array(rotated_data, input_image.width, input_image.height);

    // (w − i − 1, h − j − 1).
    for (col, row, _pixel) in input_image.iter_col_major(){
        // --> eprintln!("{},{}, {}",input_image.height, row, col);
        let pixel = array.get_element( input_image.width - col - 1, input_image.height - row - 1);
        *pixel = _pixel.clone();
    }
    
    // let elapsed = now.elapsed();
    // eprintln!("{:.2?}", elapsed);
    
    array
}