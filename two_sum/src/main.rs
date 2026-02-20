use std::io;

fn main() {
    let mut nums = vec![0];

    for i in 0..5{
        nums.push(i);
    }
    
    let mut input = String::new();
    io::stdin().read_line(&mut input ) .expect("failed");
    let number : i32 = input.trim().parse().unwrap(); 
    nums.push(number);
    println!("{:?}", nums);
}
