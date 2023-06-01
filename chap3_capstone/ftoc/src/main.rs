use std::io;

fn main() {
    println!("Please input a temprature in Fahrenheit.");

    let mut temp_fahrenheit = String::new();

    io::stdin()
        .read_line(&mut temp_fahrenheit)
        .expect("Failed to read line");

    let temp_fahrenheit: u32 = temp_fahrenheit.trim().parse().expect("Didn't find a number!");

    let temp_celsius = (temp_fahrenheit - 32) * 5/9;
    println!("{temp_fahrenheit} Fahrenheit is {temp_celsius} in Celsius!")
}
