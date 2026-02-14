use ndarray::prelude::*; // a preludge is the subset of the library that is commonly used function and macros, so we can import it all at once
use ndarray::Array; /* this includes a macro that allows us to create arrays of matrix of higher dimensions easily 
macros are functions that ends with "!" a function has fixed number of parameters while macros has variable number of parameters 
variable number of parmeters it is possible cause instead of directly running the app at compile time it expands at the compile time
 while a function is executed at runtime   */ 

 fn main(){
    let arr = array![[1, 2, 3], [4, 5, 6], [7, 8, 9]]; 
    //this is how macro is array is expands at compile time 
    /*let m = {
    // 1. Create a flat "chain" of data (the rule of chain!)
    let data = vec![1.0, 2.0, 3.0, 4.0];
    
    // 2. Define the shape (2 rows, 2 columns)
    let shape = (2, 2);
    
    // 3. Call the low-level constructor
    // The macro handles the "unwrap" safely for you
    ndarray::Array2::from_shape_vec(shape, data)
        .expect("Macro failed to create array: check your brackets!")

    };*/

    
 }
