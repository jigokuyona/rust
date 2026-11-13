use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num1 = &args[1];
    let num2 = &args[2];
    let float_num1: f32 = num1.parse::<f32>().unwrap();
    let float_num2: f32 = num2.parse::<f32>().unwrap();
    let result = sum(float_num1, float_num2).to_string();
    println!("Result of {num1} + {num2} = {result}")
}

fn sum(num1: f32, num2: f32) -> f32 {
    return num1 + num2;
}