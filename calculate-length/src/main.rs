use std::io;

fn main() {
    let mut user_input: String = String::new();
    println!("Enter your text here");
    io::stdin().read_line(&mut user_input).unwrap();
    let length = calculate_length(&user_input);
    println!("The string is '{user_input}' and length is '{length}'");
}

fn calculate_length(a_string: &String) -> usize {
    let length = a_string.len();
    length
}
