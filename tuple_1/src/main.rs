use std::io;
fn main() {
    let mut input: String = String::new();
    println!("Enter three numbers:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let numbers_in_vector: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("The number format is wrong"))
        .collect();
    let my_tuple: (u32, u32, u32) = (
        numbers_in_vector[0],
        numbers_in_vector[1],
        numbers_in_vector[2],
    );
    println!("{:?}", my_tuple);
}
