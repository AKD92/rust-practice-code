fn main() {
    let mut s = String::from("take ownership");
    take_ownership(s);
    s = String::from("another string");
    println!("{s}");
    let i: i32 = -455;
    make_copy(i);
}

fn take_ownership(mut some_string: String) {
    some_string.push_str("new hello");
    println!("{some_string}");
}

fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}
