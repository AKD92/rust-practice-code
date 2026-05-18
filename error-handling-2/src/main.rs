
use std::{fs::File, io::ErrorKind};

fn main() {
    let file_open_result = File::open("D:\\RTL_DEV_2");
    let file: File;
    match file_open_result {
        Ok(fl) => {
            file = fl;
        },
        Err(error) => {
            match error.kind() {
                ErrorKind::IsADirectory => {
                    println!("The file is actually a directory");
                },
                x => {
                    println!("Some other form of error happened: {x:?}");
                }
            }
        }
    }
}