use std::io;

fn main() {
    let name: String = String::from("Hello, world");
    let (monkey, monkeysizes) = returning_tuple(name);
    println!("Here we have {monkey} with size {monkeysizes}");
}

fn returning_tuple(s: String) -> (String, usize) {
    let size: usize = s.len();
    (s, size)
}
