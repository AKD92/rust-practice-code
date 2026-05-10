fn main() {
    let mut st: String = String::new();
    st.push_str("Hello There ..?");
    create_slice(st);
}

// fn create_slice() -> &str {
//     let st: String = String::new();
//     st.push_str("Hello There ..?");
//     let slice1: &str = &st[0..5];
//     slice1;
// }

fn create_slice(s: String) -> &str {
    let slice1: &str = &s[0..5];
    println!("The slice contains: {slice1}");
    slice1
}
