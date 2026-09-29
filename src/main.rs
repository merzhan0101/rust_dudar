fn main() {
    let mut s = String::from("Hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2); // последнее использование r1 и r2

    let r3 = &mut s;            // теперь можно
    r3.push_str(", world");
    println!("{}", r3);
}