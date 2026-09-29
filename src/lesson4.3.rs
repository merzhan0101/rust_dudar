use std::panic::PanicHookInfo;

fn main() {
    let mut s = String::from("Hello");
    change(&mut s); // Изменяемая заимствование
    
    println!("{}", s);
}

fn change(s: &mut String) {
    s.push_str(", World");
} 