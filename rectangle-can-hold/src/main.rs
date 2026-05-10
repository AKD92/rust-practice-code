
use std::io;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(self: &Self, other: &Rectangle) -> bool {
        let can_hold: bool = self.width >= other.width && self.height >= other.height;
        can_hold
    }
}

fn main() {
    let mut user_input: String = String::new();
    println!("Enter widht and height of rectangle A, separated by space");
    io::stdin().read_line(&mut user_input).unwrap();
    let mut input_parts = user_input.split_whitespace();
    let rectangle_a: Rectangle = Rectangle {
        width: input_parts.next().unwrap().parse::<u32>().unwrap(),
        height: input_parts.next().unwrap().parse::<u32>().unwrap()
    };
    user_input.clear();
    println!("Enter widht and height of rectangle B, separated by space");
    io::stdin().read_line(&mut user_input).unwrap();
    input_parts = user_input.split_whitespace();
    let rectangle_b: Rectangle = Rectangle {
        width: input_parts.next().unwrap().parse::<u32>().unwrap(),
        height: input_parts.next().unwrap().parse::<u32>().unwrap()
    };
    let can_hold: bool = rectangle_a.can_hold(&rectangle_b);
    let can_hold_display: &str;
    if can_hold {
        can_hold_display = "YES";
    } else {
        can_hold_display = "NO";
    }
    println!("Can rectangle A hold rectangle B? {can_hold_display}");
}