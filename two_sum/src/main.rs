use std::io;

fn main() {
    let mut nums = Vec::new();
    nums.push(5);


    let mut input = String::new();

    // for i in 0..{
    //     nums.push(i);
    // }
    let mut input = String::new();
    for line in io::stdin().lines(){
        let line = match line {
            Ok(l) => l,
            Err(_) => break, // Exit loop on IO error
        };
        match line.trim().parse::<i32>() {
            Ok(number) => {
                nums.push(number);
            }

    }
    
    
    
    
    println!("{:?}", nums);
}
}