
use std::io;
use std::string;

fn compute_area(width: u32, height: u32) -> u32 {
    let area: u32 = width * height;
    area
}

fn main() {
    let mut user_input: string::String = String::new();
    println!("Enter width and height, separated by space");
    io::stdin().read_line(&mut user_input).unwrap();
    let mut input_parts = user_input.split_whitespace();
    let width: u32 = input_parts.next().unwrap().parse::<u32>().unwrap();
    let height: u32 = input_parts.next().unwrap().parse::<u32>().unwrap();
    let area = compute_area(width, height);
    println!("The area of the rectangle: {area}");
}