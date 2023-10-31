# CSC411_assignment3

Sam Calise and Claudia Deverdits

We received help from TAs Ayman, Vincent, and Oliver while implementing our solution. We would also like to note that we have opted to use an Array2 solution that is designed by students who have previously taken CSC 411 instead of using our own implementation. We believe we have correctly implemented rotate 90 and rotate 180, and have decided not to implement the extra credit options. 

We have spent approximately 12 hours on this assignment.

### Architecture
Our solution has a thething.rs file which includes all of our rotation functions and benchmarking. Our main.rs handles CLAP which allows us to take the inputted commands and call the correct function for which transformation needs to be completed. The main function creates an initial Array2 image which holds the ppm that transformations will occur on. Then, we check whether to use row-major or col-major iteration and within each if/else if statement, we have a match statement to determine whether there will be a 90 or 180 degree rotation. If the commands entered don't match either of these options, we print to stderr and end with a nonzero exit code.

### Part C

                 | Row-major access | Col-Major Access|
*******************************************************
|90-deg rotation |    XXXXms         |     74.37ms    |
*******************************************************
|180-deg rotation|     54.81ms       |     60.26ms    |
*******************************************************
Our measurements are slightly different from our predictions we made in Part B.

### Part D
A possible memory layout that could have better cache locality than our Array2 representation is a 2D vec that groups elements to minimize cache misses when accessing adjacent elements. In the approach, we would group elements in square submatrices of size K x K, where K is a size that would fit best with the given cache size. This would work better than our 1D vector representation for 90-degree rotations because it minimizes cache misses and enhances spatial locality.
