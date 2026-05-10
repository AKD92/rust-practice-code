use std::io;

fn main() {
    let mut user_input: String = String::new();
    println!("Enter how much time you want to loop");
    io::stdin().read_line(&mut user_input).unwrap();
    let count: u32 = user_input.trim().parse::<u32>().unwrap();
    println!("Enter your stepping >= 1");
    user_input.clear();
    io::stdin().read_line(&mut user_input).unwrap();
    let stepping: u32 = user_input.trim().parse::<u32>().unwrap();
    let mut index: u32 = 0;
    loop {
        println!("You are iterating at {index}");
        index += stepping;
        if index >= count {
            break;
        }
    }
}
