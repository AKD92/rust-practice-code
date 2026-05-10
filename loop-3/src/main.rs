use std::io;

fn main() {
    let mut user_input: String = String::new();
    println!("Enter how many steps before LIFTOFF");
    io::stdin().read_line(&mut user_input).unwrap();
    let steps: u32 = user_input.trim().parse::<u32>().unwrap();
    let mut counter: u32 = steps;
    while counter != 0 {
        println!("Counting {counter}");
        counter -= 1;
    }
    println!("LIFTOFF!!!");
}
