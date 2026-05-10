use std::io;

fn main() {
    let mut user_input: String = String::new();
    print!("Input your age ");
    io::stdin().read_line(&mut user_input).unwrap();
    let age: u32 = user_input.trim().parse::<u32>().unwrap();
    let in_middle_age: bool = if age < 50 { false } else { true };
    if in_middle_age {
        println!("Your age is {age}, and you are in middle age");
    } else {
        println!("Your age is {age}, and you are not in middle age");
    }
}
