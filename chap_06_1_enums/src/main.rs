/* fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
*/

/* enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    println!("Hello, world!");
}
 */

/*  enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

 fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
} */

/* #[derive(Debug)] // so we can inspect the state in a minute
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

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
} */

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
} */

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

// From the Onwership Inventory #1 quiz.
fn get_or_default2(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone()
    }
}

fn get_or_default3(arg: &mut Option<String>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.as_mut().unwrap();
    s.clone()
}

fn get_or_default4(arg: &Option<&str>) -> String {
    if arg.is_none() {
        return String::new();
    }
    let s = arg.unwrap();
    s.to_string()
}