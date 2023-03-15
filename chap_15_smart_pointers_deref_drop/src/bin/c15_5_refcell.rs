/*
fn main() {
    let x = 5;
    let y = &mut x;
}
*/

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

/*

IMPORTANT

From chapter 15.5 of the Rust book:
 | We can’t modify the MockMessenger to keep track of the messages, because the
 | send method takes an immutable reference to self. We also can’t take the
 | suggestion from the error text to use &mut self instead, because then the
 | signature of send wouldn’t match the signature in the Messenger trait definition
 | (feel free to try and see what error message you get).
 |
 | This is a situation in which interior mutability can help! We’ll store the sent_messages
 | within a RefCell<T>, and then the send method will be able to modify sent_messages to
 | store the messages we’ve seen. Listing 15-22 shows what that looks like:
 */

/*
    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }
*/

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            // This also needs to be commented out, as the failure occurs when the
            // second mutable borrow is requested, and not just when it is used.
            //let mut two_borrow = self.sent_messages.borrow_mut();

            // Two simultaneous mutable borrows on a `RefCell`: accepted at compile-time,
            // rejected at runtime
            // |
            // | thread 'tests::it_sends_an_over_75_percent_warning_message' panicked at
            // | 'already borrowed: BorrowMutError'
            // |
            one_borrow.push(String::from(message));
//            two_borrow.push(String::from(message));

            //self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a before = {:?}", a);
    println!("b before = {:?}", b);
    println!("c before = {:?}", c);

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

use std::cell::UnsafeCell;

struct BadRefCell<T>(UnsafeCell<T>);

impl<T> BadRefCell<T> {

    pub fn borrow_mut(&self) -> &mut T {

        unsafe { &mut *self.0.get() }

    }

}