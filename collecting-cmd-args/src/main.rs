use std::env;
use std::process;
use std::str;

fn main() {
    let mut numbers = Vec::new();
    for arg in env::args() {
        let argument = arg;
        numbers.push(argument);
    }
    for arg in numbers {
        println!("Argument : {arg}");
    }
}
