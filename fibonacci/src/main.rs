use std::io;

fn main() {
    println!("Fibonacci Number Generator");
    println!("Please input the index of the Fibonacci row:")

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");    

    let index: i32 = input.trim().parse().expect("Please type a number");

    let mut prev2: i32 = 0;
    let mut prev1: i32 = 1;
    let mut current: i32 = prev2 + prev1;

    
}

fn gen_fib_num(num: i32) -> 32 {
    match num
}
