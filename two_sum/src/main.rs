use std::io;

fn main() {
    let mut nums = vec![];
    for(int i )
    let mut input = String::new();
    io::stdin().read_line(&mut input ) .expect("failed");
    let number : i32 = input.trim().parse().unwrap(); 
    nums.push(number);
    println!("{:?}", nums);
}
