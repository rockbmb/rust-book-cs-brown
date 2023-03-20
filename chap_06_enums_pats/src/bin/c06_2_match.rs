#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main1() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

/*
fn main() {
    let opt: Option<String> =
        Some(String::from("Hello world"));


    match opt {
        // _ became s
        Some(s) => println!("Some: {}", s),
        None => println!("None!")
    };

    // The code errors because of a borrow of a partially moved value.
    // To fix it, the `s` in `Some` needs to be passed by reference.
    println!("{:?}", opt);
}
*/

fn main() {
    let opt: Option<String> =
        Some(String::from("Hello world"));

    match &opt {
        // _ became s
        Some(s) => println!("Some: {}", s),
        None => println!("None!")
    };

    // The code errors because of a borrow of a partially moved value.
    // To fix it, the `s` in `Some` needs to be passed by reference.
    println!("{:?}", opt);
}