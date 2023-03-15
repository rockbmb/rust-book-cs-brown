fn main1() {
    let b = Box::new(5);
    println!("b = {}", b);
}

/*
Fails because of a lack of indirection: Rust cannot have unboxed recursive types.

enum List {
    Cons(i32, List),
    Nil,
}
 */

 enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}