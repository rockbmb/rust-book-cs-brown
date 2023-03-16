use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }

    fn recursive_print(&self) {
        match self {
            Cons(n, item) => {
                print!("list's next item = {:?};; ", n);
                let l = &**item.borrow();
                l.recursive_print();
            }
            Nil => println!("Nil!"),
        }
    }

    fn bounded_recursive_print(&self, count : u32) {
        if count == 0 {
            return
        };

        match self {
            Cons(n, item) => {
                print!("list's next item = {:?};; ", n);
                let l = &**item.borrow();
                l.bounded_recursive_print(count - 1);
            }
            Nil => println!("Nil!"),
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    b.recursive_print();

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    /*
    Unrolled recursion; before writing the recursive method, I wished
    to try and unroll it manually here to see how many dereferences would
    be needed.
     match &*a {
        Cons(n1, item1) => {
            println!("a's next item = {:?}", n1);
            match &**item1.borrow_mut() {
                Cons(n2, item2) => {
                    println!("a's next item = {:?}", n2);
                },
                Nil => {},
            }
        },
        Nil => {},
    }
    */

    a.bounded_recursive_print(10);
    b.bounded_recursive_print(10);

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    //println!("a next item = {:?}", a.tail());
}