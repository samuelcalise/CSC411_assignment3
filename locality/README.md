# CSC411_assignment3

Sam Calise and Claudia Deverdits

We received help from TAs Ayman, Vincent, and Oliver while implementing our solution. We would also like to note that we have opted to use an Array2 solution that is designed by students who have previously taken CSC 411 instead of using our own implementation. We believe we have correctly implemented rotate 90 and rotate 180 on perfect squares; however, we were unable to get our program working with images or other shapes (ie. moss.ppm).

We have spent approximately 12 hours on this assignment.

### Architecture
Our solution has a thething.rs file which includes all of our rotation functions and benchmarking. Our main.rs handles CLAP which allows us to take the inputted commands and call the correct function for which transformation needs to be completed. The main function creates an initial Array2 image which holds the ppm that transformations will occur on. Then, we check whether to use row-major or col-major iteration and within each if/else if statement, we have a match statement to determine whether there will be a 90 or 180 degree rotation. If the commands entered don't match either of these options, we print to stderr and end with a nonzero exit code.

### Part C

|                  |Row-major      |Col-major |
|------------------|---------------|----------|
|90-deg rotation   |53.10ms        |83.97ms   |
|180-deg rotation  |49.56ms        |140.05ms  |

Our measurements match our predictions from Part B of our design. 

These measurements are a result of running our program 3 times using the moss.ppm image and averaging the benchmarks. Both our design and actual benchmarks describe a speed ranking of 180 Row-major, 90 Row-major, 90 Col-major, and 180 Col-major, from fastest to slowest. 

180 Row-major is the fastest rotation because our Array2 is stored in row-major order, resulting in the most hits for loads and stores. 90 Row-major is the next fastest rotation, again because Array2's storage type is row-major order. However, in this case, there would be a high hit rate for loads and a low hit rate for stores because each pixel would need to be stored in a new column. 90 Col-major is the second slowest rotation and is the opposite of 90 Row-major where it has a low hit rate for loads but a high store rate. Finally, 180 Col-major is the slowest rotation because it misses all loads and stores.

### Part D
A possible memory layout that could have better cache locality than our Array2 representation is a 2D vec that groups elements to minimize cache misses when accessing adjacent elements. In the approach, we would group elements in square submatrices of size K x K, where K is a size that would fit best with the given cache size. This would work better than our 1D vector representation for 90-degree rotations because it minimizes cache misses and enhances spatial locality.
