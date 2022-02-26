use std::io;
use std::io::prelude::*;

fn main() {
    println!("Hello, mars!");

    print!("Enter your weight on Earth (kg): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let earth_weigth:f32 = input.trim().parse().unwrap();
    let mars_weight = calculate_weigth_on_mars(earth_weigth);
    println!("Your weight on mars: {} kg or {} g.", mars_weight, mars_weight * 1000.0);
}

fn calculate_weigth_on_mars(weigth: f32) -> f32 {
    (weigth / 9.81) * 3.771
}

// 1. Each value in Rust is owned by a variable.
// 2. When the owner goes out of scope, the value will be deallocated.
// 3. There can only be ONE owner at a time.