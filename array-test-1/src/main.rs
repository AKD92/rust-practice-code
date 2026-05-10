use std::io;

fn main() {
    let my_array = [0, 1, 2, 3, 4];
    let mut v: Vec<i32> = Vec::<i32>::new();
    let mut input = String::new();
    let stdin = io::stdin();
    let read_status = stdin.read_line(&mut input);
    let input: usize = input.trim().parse().expect("enter correct value");
    let element = my_array[input];
    println!("The element is {element}");
}
