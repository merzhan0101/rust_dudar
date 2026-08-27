// LESSON 3 => https://itproger.com/course/rust/3 

fn main() {
    // Const, tuple, arrays
    const USER_MAX_SCORE: u32 = 1_000_000;
    // println!("Info: {}", USER_MAX_SCORE);

    // КОРТЕЖ - Tuple — это способ хранить несколько значений разных типов в одной переменной.
    let mut user_alex: (i8, bool, f32, char) = (42, true, 1.86, 'R');
    user_alex.2 = 2.0;
    // println!("Info: {}", user_alex.2);

    // ARRAY - МАССИВ
    let mut nums: [i8; 5] = [1, 5, 2, 7, 3];
    nums[0] = 10;
    // println!("Info: {}", nums[0]);
}