use std::io;

fn main() {
    let mut user_input: String = String::new();
    println!("Enter how many times you want to iterate, and multiplier, separated by space.");
    io::stdin().read_line(&mut user_input).unwrap();
    let user_numbers: Vec<u32> = user_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let iter_count: u32 = user_numbers[0];
    let multiplier: u32 = user_numbers[1];
    let mut current_itr: u32 = 0;
    let mut accumulation: u32 = 0;
    let result: u32 = loop {
        accumulation = accumulation + (current_itr * multiplier);
        current_itr += 1;
        if current_itr == iter_count {
            break accumulation;
        }
    };
    println!("Your accumulaton is {result}");
}
