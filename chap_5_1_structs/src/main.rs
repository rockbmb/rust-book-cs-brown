struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/* fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
} */

/* fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
} */

/* fn main() {
   let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
   };
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
} */

/* fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
     };
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    user1.email;
} */

struct Point {
    x: i32,
    y: i32,
  }

fn main() {
    let mut a = Point { x: 1, y: 2 };
    a.x += 1;
    let b = Point { y: 1, ..a };
    a.x += 1;
    println!("{}", b.x);
}