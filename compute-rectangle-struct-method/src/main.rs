
use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        println!("The rectangle is {self:?}");
        self.width * self.height
    }
    fn create(size: u32) -> Self {
        Self { width: size, height: size }
    }
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
    let mut area: u32 = dimension.area();
    println!("The area of the rectangle is {area}");
    println!("The rectangle is {dimension:#?})");
    let refe: &Rectangle = &dimension;
    area = refe.area();
    println!("The area of refe referencing to dimension is {area}");

    let square = Rectangle::create(5);
    area = square.area();
    println!("The area of the rectangle is {area}");
}