use std::io;

fn main() {
    let mut s: String = String::new();
    append_text(&mut s);
    println!("After appending, the string is: {s}");
}

fn append_text(some_string: &mut String) {
    some_string.push_str("Hello string, pushing you in");
}
