
use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn compute_area(dimension: &Rectangle) -> u32 {
    let area: u32 = dimension.width * dimension.height;
    area
}

fn main() {
    let mut user_input: String = String::new();
    println!("Enter width and height of the rectangle, seperated by space");
    io::stdin().read_line(&mut user_input).unwrap();
    let mut input_parts = user_input.split_whitespace();
    let dimension: Rectangle = Rectangle {
        width: input_parts.next().unwrap().parse::<u32>().unwrap(),
        height: input_parts.next().unwrap().parse::<u32>().unwrap()
    };
    let area: u32 = compute_area(&dimension);
    println!("The area of the rectangle is {area}");
    println!("The dimension is {dimension:#?}");
}