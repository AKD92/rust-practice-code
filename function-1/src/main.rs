fn main() {
    let y = {
        let x = 32;
        let z = 3;
        x + z
    };
    print_number(y, y as u32);
    let returning_value: i32 = returns_nothing();
    println!("The 'returns_nothing' returned {returning_value}");
}

fn print_number(x: i32, y: u32) {
    println!("Value of x is {x} and value of y is {y}");
}

fn returns_nothing() -> i32 {
    5
}
