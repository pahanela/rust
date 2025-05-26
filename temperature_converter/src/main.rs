use std::io;

fn main() {
    println!("Temperature Units Converter");
    println!("Please input the temperature value in Fahrenheit:");

    let mut input: String = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input_f: f64 = input.trim().parse().expect("Please type a number!");

    let output_c: f64 = convert_f2c(input_f);

    println!("The corresponding temperature value in Celsius:\n{output_c}");
}

fn convert_f2c(temp: f64) -> f64 {
    (temp - 32.0) * 5.0 / 9.0
}

fn convert_c2f(temp: f64) -> f64 {
    temp * 9.0 / 5.0 + 32.0
}
