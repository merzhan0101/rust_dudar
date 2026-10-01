// LESSON #4 – Управление памятью и владение (Ownership)
use std::io;

fn main() {
    // User input
    // let mut user_data = String::new();
    // io::stdin().read_line(&mut user_data).expect("Failed to read input");
    // println!("Result: {}", user_data);

    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("Enter first number: ");
    io::stdin().read_line(&mut num1).expect("Failed to read information");

    println!("Enter second number: ");
    io::stdin().read_line(&mut num2).expect("Failed to read information");

    let data1: i16 = num1.trim().parse().expect("Please enter a valid number");
    let data2: u8 = num2.trim().parse().expect("Please enter a valid number");

    println!("Result1: {}, result2: {}", data1, data2);

    let mut res: i16 = data1 + data2 as i16;
    println!("Result: {}", res);

    res += 1;
    println!("Result: {}", res);
}