
use std::{fs::File, io::{self, Read}};

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let file_open_result = File::open(filename);
    let mut file = match file_open_result {
        Ok(fl) => fl,
        Err(error) => return Err(error)
    };
    let mut username: String = String::new();
    let read_result = file.read_to_string(&mut username);
    match read_result {
        Ok(_) => Ok(username),
        Err(error) => Err(error)
    }
}

fn main() {
    let filename: &str = "D:\\RSources\\LICENSE5";
    let username: String = read_username_from_file(filename)
        .expect("Error occured while reading file");
    println!("The username is: {username}");
}