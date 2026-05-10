
use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let mut user_input: String = String::new();
    println!("Enter the size of square you want to create");
    io::stdin().read_line(&mut user_input).unwrap();
    let square: Rectangle = Rectangle::square(user_input.trim().parse::<u32>().unwrap());
    let area: u32 = square.area();
    println!("The area of the square is {area}");
    println!("The square itself is {square:?}");
}