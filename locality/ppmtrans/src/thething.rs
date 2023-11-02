use array2::Array2;
use csc411_image::Rgb;
//use std::time::Instant;


/// Function: Rotate Row Major 90
///
/// The invariant in this function is getting a pixel at a certain coordinate
/// and "pushing" the pixel into an updated array2.
///
/// The function declares a `rotated_data` vector which is a 1d vector of
/// `Rbg` black (0,0,0) pixels. The length of the vector is determined by
/// the calculation of the passed in `input_image`'s width and length
/// that is set form `main.rs`'s function call to `rotate_rowmajor_90`.
/// Then a mutable array is declared with is an `Array2` object whose
/// pixels are the all black pixels from `rotated_data`. When iterating
/// through `iter_row_major` function, a pixel variable is located 
/// via `input_image.height - row - 1, col` which is a black image pixel
/// that recieves a new value `_pixel` which is the cloned colored RGB of 
/// the initial image from command line. Then, `array` is returned from
/// the function which is an `Array2<Rbg>` that is the rotated pixels 
/// determined by the coordinate images from `Array2`'s function 
/// `get_element`.
///
/// Parameter:
///
/// `input_image: &Array2<Rgb>` := the initial 1 dimensional vector of the
/// RBG pixels from the ppm file
///
/// Function Return Value:
///
/// array := `Array2` object that returns `Array2<Rgb>` that is the rotated
/// pixels based on degree provided by command line arguments
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
/// The function declares a `rotated_data` vector which is a 1d vector of
/// `Rbg` black (0,0,0) pixels. The length of the vector is determined by
/// the calculation of the passed in `input_image`'s width and length
/// that is set form `main.rs`'s function call to `rotate_rowmajor_180`.
/// Then a mutable array is declared with is an `Array2` object whose
/// pixels are the all black pixels from `rotated_data`. When iterating
/// through `iter_row_major` function, a pixel variable is located 
/// via `input_image.width - col - 1, input_image.height - row - 1` which is a black image pixel
/// that recieves a new value `_pixel` which is the cloned colored RGB of 
/// the initial image from command line. Then, `array` is returned from
/// the function which is an `Array2<Rbg>` that is the rotated pixels 
/// determined by the coordinate images from `Array2`'s function 
/// `get_element`.
///
/// Parameter:
///
/// `input_image: &Array2<Rgb>` := the initial 1 dimensional vector of the
/// RBG pixels from the ppm file
///
/// Function Return Value:
///
/// array := `Array2` object that returns `Array2<Rgb>` that is the rotated
/// pixels based on degree provided by command line arguments
pub fn rotate_rowmajor_180(input_image: &Array2<Rgb>) -> Array2<Rgb> {

    let rotated_data = vec![Rgb{red: 0,green: 0, blue: 0};input_image.width * input_image.height];
    // let now = Instant::now();
    
    let mut array = Array2::new_array(rotated_data, input_image.width, input_image.height);

    for (col, row, _pixel) in input_image.iter_row_major(){
        // --> eprintln!("{},{}, {}",input_image.height, row, col);
        let pixel = array.get_element( input_image.width - col - 1, input_image.height - row - 1);
        *pixel = _pixel.clone();
    }
    
    // let elapsed = now.elapsed();
    // eprintln!("{:.2?}", elapsed);
    
    array
}

/// Function: Rotate Col Major 90
///
/// The invariant in this function is getting a pixel at a certain coordinate
/// and "pushing" the pixel into an updated array2.
///
/// The function declares a `rotated_data` vector which is a 1d vector of
/// `Rbg` black (0,0,0) pixels. The length of the vector is determined by
/// the calculation of the passed in `input_image`'s width and length
/// that is set form `main.rs`'s function call to `rotate_colmajor_90`.
/// Then a mutable array is declared with is an `Array2` object whose
/// pixels are the all black pixels from `rotated_data`. When iterating
/// through `iter_row_major` function, a pixel variable is located 
/// via `input_image.height - row - 1, col` which is a black image pixel
/// that recieves a new value `_pixel` which is the cloned colored RGB of 
/// the initial image from command line. Then, `array` is returned from
/// the function which is an `Array2<Rbg>` that is the rotated pixels 
/// determined by the coordinate images from `Array2`'s function 
/// `get_element`.
///
/// Parameter:
///
/// `input_image: &Array2<Rgb>` := the initial 1 dimensional vector of the
/// RBG pixels from the ppm file
///
/// Function Return Value:
///
/// array := `Array2` object that returns `Array2<Rgb>` that is the rotated
/// pixels based on degree provided by command line arguments
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

/// Function: Rotate Col Major 180
///
/// The invariant in this function is getting a pixel at a certain coordinate
/// and "pushing" the pixel into an updated array2.
///
/// The function declares a `rotated_data` vector which is a 1d vector of
/// `Rbg` black (0,0,0) pixels. The length of the vector is determined by
/// the calculation of the passed in `input_image`'s width and length
/// that is set form `main.rs`'s function call to `rotate_colmajor_180`.
/// Then a mutable array is declared with is an `Array2` object whose
/// pixels are the all black pixels from `rotated_data`. When iterating
/// through `iter_row_major` function, a pixel variable is located 
/// via `input_image.width - col - 1, input_image.height - row - 1` which is a black image pixel
/// that recieves a new value `_pixel` which is the cloned colored RGB of 
/// the initial image from command line. Then, `array` is returned from
/// the function which is an `Array2<Rbg>` that is the rotated pixels 
/// determined by the coordinate images from `Array2`'s function 
/// `get_element`.
///
/// Parameter:
///
/// `input_image: &Array2<Rgb>` := the initial 1 dimensional vector of the
/// RBG pixels from the ppm file
///
/// Function Return Value:
///
/// array := `Array2` object that returns `Array2<Rgb>` that is the rotated
/// pixels based on degree provided by command line arguments
pub fn rotate_colmajor_180(input_image: &Array2<Rgb>) -> Array2<Rgb> {
    
    let rotated_data = vec![Rgb{red: 0,green: 0, blue: 0};input_image.width * input_image.height];
    // let now = Instant::now();
    
    let mut array = Array2::new_array(rotated_data, input_image.width, input_image.height);

    for (col, row, _pixel) in input_image.iter_col_major(){
        // --> eprintln!("{},{}, {}",input_image.height, row, col);
        let pixel = array.get_element( input_image.width - col - 1, input_image.height - row - 1);
        *pixel = _pixel.clone();
    }
    
    // let elapsed = now.elapsed();
    // eprintln!("{:.2?}", elapsed);

    array
}