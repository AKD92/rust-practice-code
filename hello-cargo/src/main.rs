const HELLO_B: u64 = 56;
fn main() {
    let n: u64 = 15;
    let n2 = n + 1;
    let n3 = &n2;
    gcd(6, n3);
}

fn gcd(mut m: u64, n: &u64) -> u64 {
    while (m < *n) {
        println!("{m} Hello Cargo!");
        m += 1;
    }
    m
}
