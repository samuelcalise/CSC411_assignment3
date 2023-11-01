use array2::Array2;
use csc411_image::Rgb;
//use std::time::Instant;

/*

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
    
    // let now = Instant::now();

    // if input_image.height == input_image.width //Assume sqaure
    // {
        for (col, row, _pixel) in input_image.iter_col_major(){
            let pixel = input_image.get_element(input_image.height - row - 1, col);
            rotated_data.push(Rgb {
                red: pixel.red,
                green: pixel.green,
                blue: pixel.blue,
            });
        }
    //}
    // else // Assume rectangle
    // {
    //     for (col, row, _pixel) in input_image.iter_col_major(){
    //         //let pixel = input_image.get_element(col, input_image.height - row - 1);
    //         let pixel = input_image.get_element(input_image.height - row - 1, col);
    //         rotated_data.push(Rgb {
    //             red: pixel.red,
    //             green: pixel.green,
    //             blue: pixel.blue,
    //         });
    //     }
    // }
    
    // let elapsed = now.elapsed();
    // eprintln!("{:.2?}", elapsed);

    Array2::new_array(rotated_data, input_image.height, input_image.width)
}

*/



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
    let mut rotated_data = vec![Rgb{red: 0,green: 0, blue: 0};input_image.width * input_image.height];
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


/*

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
    // let now = Instant::now();
    for (col, row, _pixel) in input_image.iter_row_major(){
        let pixel = input_image.get_element( input_image.height - row - 1, input_image.width - col - 1);
        rotated_data.push(Rgb {
            red: pixel.red,
            green: pixel.green,
            blue: pixel.blue,
        });
    }
    // let elapsed = now.elapsed();
    // eprintln!("{:.2?}", elapsed);
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
    if input_image.height == input_image.width //Assume sqaure
    {
        // let now = Instant::now();
        for (row, col, _pixel) in input_image.iter_row_major(){
            let pixel = input_image.get_element( input_image.height - col - 1, input_image.width - row - 1);
            rotated_data.push(Rgb {
                red: pixel.red,
                green: pixel.green,
                blue: pixel.blue,
            });
        }
    }
    else //Assume Rectangle
    {
        // let now = Instant::now();
        for (row, col, _pixel) in input_image.iter_row_major(){
            let pixel = input_image.get_element(input_image.width - row - 1, input_image.height - col - 1);
            rotated_data.push(Rgb {
                red: pixel.red,
                green: pixel.green,
                blue: pixel.blue,
            });
        }
    }
   
    // let elapsed = now.elapsed();
    // eprintln!("{:.2?}", elapsed);
    Array2::new_array(rotated_data, input_image.height, input_image.width)
}
*/