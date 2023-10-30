use array2::Array2;
use csc411_image::Rgb;



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
    let mut rotated_data = Vec::new();

    for (col, row, _pixel) in input_image.iter_col_major(){
        let pixel = input_image.get_element(input_image.height - row - 1, col);
        rotated_data.push(Rgb {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        });
    }

    Array2::new_array(rotated_data, input_image.height, input_image.width)
}

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
    let mut rotated_data = Vec::new();

    for (row, col, _pixel) in input_image.iter_row_major(){
        let pixel = input_image.get_element(input_image.height - row - 1, col);
        rotated_data.push(Rgb {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        });
    }

    Array2::new_array(rotated_data, input_image.height, input_image.width)
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
    let mut rotated_data = Vec::new();

    for (col, row, _pixel) in input_image.iter_row_major(){
        let pixel = input_image.get_element( input_image.height - row - 1, input_image.width - col - 1,);
        rotated_data.push(Rgb {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        });
    }

    Array2::new_array(rotated_data, input_image.height, input_image.width)
}

/// Function: Rotate Column Major 180
///
/// The invariant in this function is getting a pixel at a certain coordinate
/// and "pushing" the pixel into an updated array2.
///
/// This function's parameter is taking the input image from the command line
/// and references it's array2 1d vector. When iterating through the vector,
/// the function will return an `array2<Rgb>` which is the expected rotated
/// image for 180 degrees clockwise rotation in row_major iteration.
pub fn rotate_rowmajor_180(input_image: &Array2<Rgb>) -> Array2<Rgb> {

    let mut rotated_data = Vec::new();

    for (row, col, _pixel) in input_image.iter_row_major(){
        let pixel = input_image.get_element( input_image.height - col - 1, input_image.width - row - 1,);
        rotated_data.push(Rgb {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        });
    }

    Array2::new_array(rotated_data, input_image.height, input_image.width)
}