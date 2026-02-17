use std::io; // library for input and output 

fn main() {
    println!("enter something:"); // print a message to the user 
    let mut input = String::new(); // create a mutable variable to store user input 
    io::stdin().read_line(&mut input).expect("failed to read line"); // read user input and store it in the variable
    let number: i32 = input.trim().parse().unwrap();// convert the input string to an integer //ownership of the input is not moved to the number variable, it is still owned by the input variable because parsing only reads data.
    
    
    if number > 0 { 
        println!("the number is positive"); 
    } else if number < 0 { 
        println!("the number is negative");
    } else { 
        println!("the number is zero"); 
    }

}
