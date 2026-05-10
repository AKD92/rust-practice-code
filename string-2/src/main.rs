use std::ptr::null;

fn main() {
    let mut s = String::from("hello");
    s = String::from("ahoy");
    s.push_str("..");
    let mut user_str: String;
    user_str = String::from("mullon");
    println!("{user_str}");
    println!("{s} world!");
}
