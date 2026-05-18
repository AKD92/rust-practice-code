
use std::{fs::File, io::Read};
use std::io::Error;

fn read_contents_from_file(filename: &str) -> Result<String, Error> {
    let mut file = File::open(filename)?;
    let mut contents: String = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    let filename: &str = "D:\\rsources\\license";
    let contents = read_contents_from_file(filename)
        .expect("Could not read file");
    println!("The content is\n {contents}");
}