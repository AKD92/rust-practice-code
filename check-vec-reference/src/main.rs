
use std::ptr;

fn main() {
    let mut x: u32 = 59u32;
    let mut coll: Vec<&u32> = Vec::new();
    coll.push(&x);
    let ref1 = &x;
    let ref2 = coll[0];
    if ptr::eq(ref1, ref2) {
        println!("The references are the same");
    } else {
        println!("The references are NOT the same");
    }
    let ref3: &mut u32 = &mut x;
    *ref3 = 90u32;
}
