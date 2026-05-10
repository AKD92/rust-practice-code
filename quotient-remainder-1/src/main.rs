use std::io;

fn main() {
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    let (divisor, divident): (i32, i32);
    let numbers: Vec<i32> = user_input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    divisor = numbers[0];
    divident = numbers[1];
    let quotient = (divident / divisor) as u32;
    let remainder = (divident % divisor) as u32;
    println!("The quotient is {quotient} and remainder is {remainder}");
}
