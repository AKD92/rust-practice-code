
use std::{fs::File, io::ErrorKind};

fn main() {
    let file_open_result = File::open("D:\\Rsources");
    match file_open_result {
        Ok(fl) => {},
        Err(error) => {
            if error.kind() == ErrorKind::IsADirectory {
                println!("It is a directory");
            } else if error.kind() == ErrorKind::NotFound {
                println!("The fild is not found");
            } else {
                println!("Some other error happened, {error:?}");
            }
        }
    }
}