use crate::List1::{Cons1, Nil1};

enum List1 {
    Cons1(i32, Box<List1>),
    Nil1,
}

fn main1() {
    let a = Cons1(5, Box::new(Cons1(10, Box::new(Nil1))));
    // `a` moved here, so the line following the one below will not work.
    let b = Cons1(3, Box::new(a));
    //let c = Cons(4, Box::new(a));
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

//
//
//

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main2() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}