
use std::fs::File;

fn main() {
    let file_open_result = File::open("hello.txt");
    let file = match file_open_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Could not open file {}", error.to_string());
        }
    };
    println!("File opened");
}