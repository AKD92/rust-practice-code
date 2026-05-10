use std::io;

fn main() {
    let mut user_input: String = String::new();
    println!("Input how many numbers you want to input");
    io::stdin().read_line(&mut user_input);
}
