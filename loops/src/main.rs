use std::io;

fn main() {
    let mut num = 0 ;
    let mut nums = Vec::new();
    nums.push(1);
    //infinite loop //printing odd int in range 0 to 100 
    
    let loop_var = loop {
        num = num +1 ;

        if num % 2 != 0 {
            println!("{}",num);
        }

        else if num > 101 {
            break num; // value given at break will be passed to the variable equated to loop_var 
        }
    };
    println!("this is the loop variable :");
    println!("{}",loop_var);

    let mut dig_num = 0 ; 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed !!");
    dig_num = input.trim().parse().unwrap();

    println!("enter the number you want to find digit from : {} ",dig_num);
    let mut digits = 0 ; 
    while(dig_num>0){
        digits = dig_num % 10 ;
        println!("digs = {}",digits);
        dig_num = dig_num / 10 ;
    }

    
}
