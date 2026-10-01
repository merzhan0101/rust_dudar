// LESSON #5 – Условные конструкции: if-else, match
use core::num;

fn main() {
    // Условные операторы ===============================================================
    // let number = 5;
    // let is_has_car = false;

    // if number >= 5 || is_has_car {
    //     println!("Number is bigger than 5");
    // } else if number == 4 {
    //     println!("Number is 4");
    // } else {
    //     println!("Number is 5 or smaller");
    // }

    // Тернарный оператор ===============================================================
    // let condition = true;
    // let number = if condition { 5 } else { 10 };
    // println!("Res: {}", number); 

    // Оператор match ===============================================================
    let number = 3;

    match number {
        1 => println!("Result 1"),
        2 => println!("Result 2"),
        3 => println!("Result 3"),
        4 => println!("Result 4"),
        5 => println!("Result 5"),
        _ => println!("Else"),
    }
}