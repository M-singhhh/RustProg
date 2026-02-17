use ndarray::prelude::*; // a preludge is the subset of the library that is commonly used function and macros, so we can import it all at once
use ndarray::Array; /* this includes a macro that allows us to create arrays of matrix of higher dimensions easily 
macros are functions that ends with "!" a function has fixed number of parameters while macros has variable number of parameters 
variable number of parmeters it is possible cause instead of directly running the app at compile time it expands at the compile time
 while a function is executed at runtime   */ 
use std::io; /* this is the standard library for input and output operations in rust it provides a way to read and write data to the console or to files */


 fn main(){
   println!("enter the operation you want to perform : \n1. addition \n2. subtraction \n3. multiplication \n4. dot product \n5. cross product \n6. exit");


   let arr1 = array![[1, 2, 3], [4, 5, 6], [7, 8, 9]]; 
   let arr2 = array![[10, 11, 12], [13, 14, 15], [16, 17, 18]]; 
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

   let num = 8 ;
   println!("scalling factor : {num}"); // this is a new feature in rust 1.58 called "format string capture" it allows us to use the variable name directly in the format string without having to specify it as an argument
   println!("{}", arr1); 
   println!("/////////////////////////////////////////////");
   println!("/////////////////////////////////////////////");
   println!("{:?}", arr2);  

    /* format specifer {}	Display	"Print this for a human user (clean and simple)."
       format specifer {:?}	Debug	"Print this for a developer (detailed and unambiguous)." 
       format specifer {:#?}	Pretty Debug	"Print this for a developer (detailed and pretty)." 
       format specifer {:x} LowerHex print the number as hexa dex */

   let add = &arr1 + &arr2; /* this is how we can add two arrays together this "&" means we are giveing the refrence of the array to the add function instead of giving the ownership of the array to the add function this is called borrowing in rust and it allows us to use the same array multiple times without having to worry about ownership and borrowing rules */
   println!("addition of two arrays : \n{add}");

   let sub = &arr1 - &arr2; /* this is how we can subtract two arrays together */ 
   println!("subtraction of two arrays : \n{sub}");

   let mul = &arr1 * &arr2; /* this is how we can multiply two arrays together this is called element wise multiplication or hadamard product */
   println!("multiplication of two arrays : \n{mul}"); 

   let dot = arr1.dot(&arr2); /* this is how we can do the dot product of two arrays together this is called matrix multiplication */ 
   println!("dot product of two arrays : \n{dot}"); 


   let vec1 = array![1, 2, 3]; 
   let vec2 = array![4, 5, 6]; 
   let cross = vec1.cross(&vec2); /* this is how we can do the cross product of two arrays together this is only defined for 3D vectors */ 
   println!("cross product of two arrays : \n{cross}");


 }
