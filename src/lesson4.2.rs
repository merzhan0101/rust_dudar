fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1); //Заимствование по ссылке

    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len() //Удаляем точку с запятой, так как это выражение возвращает значение
}