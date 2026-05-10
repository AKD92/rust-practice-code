
use std::io;

fn compute_area(dimension: (u32, u32)) -> u32 {
    let area: u32 = dimension.0 * dimension.1;
    area
}

fn main() {
    let mut user_input: String = String::new();
    println!("Enter width and height of rectangle separated by space");
    io::stdin().read_line(&mut user_input).unwrap();
    let mut input_parts = user_input.split_whitespace();
    let width: u32 = input_parts.next().unwrap().parse::<u32>().unwrap();
    let height: u32 = input_parts.next().unwrap().parse::<u32>().unwrap();
    let area = compute_area((width, height));
    println!("The area of the rectangle is: {area}");
}