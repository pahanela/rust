use std::io;

fn main() {
    println!("Temperature Units Converter");
    println!("Please input the temperature in Fahrenheit:");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input_f: f64 = input.trim().parse().expect("Please type a number!");

    println!("You entered: {input_f}");
}
