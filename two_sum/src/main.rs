use std::io;

fn two_sum(nums: &[i32] , target : &i32 ) {
    let mut boo = false ; 
    for i in 0 .. nums.len() {
        for j in i .. nums.len()-1 {
            if nums[j] + nums[i] == *target {
                println!("indeces of sum : {} {}",i , j );
                boo = true ; 

            }
        }
    }
    if !boo {println!("not found ");} 
}

fn main() {
    let mut nums = vec![76];
    println!("enter the number of elements in the array :");
    let mut inputNO = 35 ;
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1 ).expect("failed !!");
    inputNO = input1.trim().parse().unwrap();
    for i in 0 .. inputNO {
        let mut input2 = String::new() ; 
        io::stdin().read_line(&mut input2 ).expect("failed !!");
        let mut ioToVec = input2.trim().parse().unwrap();
        nums.push(ioToVec);
    }

    
    let mut index = -1 ; 
    println!("enter the target number :"); 
    let mut target = 35 ;
    let mut input = String::new();
    io::stdin().read_line(&mut input ).expect("failed !!");
    target = input.trim().parse().unwrap();


    
    
    
    
    two_sum(&nums , &target);
    
    
    
    
    
    
    
}
