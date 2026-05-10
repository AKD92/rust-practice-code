
enum MyOption<T> {
    None,
    Some(T),
    Other { signal_number: u8, value: i32 }
}

fn main() {
    let option1: MyOption<u32> = MyOption::None;
    let option2: MyOption<()> = MyOption::Other { signal_number: 2, value: -98 };

    // check with if..let
    if let MyOption::Some(x) = option1 {
        println!("The option1 is MyOption::Some with data {x}");
    } else if let MyOption::Other { signal_number, value} = option2 {
        println!("The option2 is MyOption::Other {signal_number} and value {value}");
    }

    // do the same as above with match construct
    match option1 {
        MyOption::Some(x) => {
            println!("Using match, option1 is MyOption::Sum with value {x}");
            println!("Another println command inside some");
        },
        MyOption::None => {
            println!("Using match, option1 is MyOption::None");
            println!("Another println command inside None"); 
        }
        _ => ()
    }
    match option2 {
        MyOption::Other {signal_number, value} => {
            println!("Using match, option2 is MyOption::Other with fields {signal_number} and {value}");
            println!("Another println command inside Other");
        },
        _ => ()
    }
}
