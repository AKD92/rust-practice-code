const SHADOW_VALUE: i32 = -32;
fn main() {
    let x: u32 = SHADOW_VALUE as u32;
    let mut x = x + 1;
    println!("The value of outer x is {x}");
    {
        let x = x * 2;
        println!("The value of inner x is {x}");
    }
    println!("The value of outer x is {x}");
}
