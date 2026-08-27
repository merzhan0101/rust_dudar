// LESSON 2 - cargo

// форматировать файл: rustfmt name_file.rs

fn main() {

    // Integer: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128 //Тип данных
    let num: u8 = 50; //mut означает mutable — изменяемый.
    println!("Result: {}", num);

    let num: i16 = -4500;
    println!("Result: {}", num);

    let num: u64 = 1000000;
    println!("Result: {}", num);

    // Float
    let num: f32 = 5.453;
    println!("Result: {}", num);

    let num: f64 = 5.453234;
    println!("Result: {}", num);

    // Boolean
    let is_has_car: bool = false;
    println!("Result: {}", is_has_car);

    // Char
    let sym: char = '%';
    println!("Result: {}", sym);
}