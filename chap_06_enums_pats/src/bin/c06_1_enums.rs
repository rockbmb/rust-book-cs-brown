fn main1() {
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

enum IpAddrKind {
    V4,
    V6,
}

fn main2() {
    println!("Hello, world!");
}

  enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

 fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}