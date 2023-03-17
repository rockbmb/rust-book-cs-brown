use std::sync::Mutex;
use std::thread;

fn main1() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m)
}

/* 
fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    /*
    IMPORTANT

    |
    | The error message states that the counter value was moved in the previous iteration
    | of the loop. Rust is telling us that we can’t move the ownership of lock counter into
    | multiple threads.
    |
     */
    for _ in 0 .. 10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
 */

/* 
use std::rc::Rc;

fn main() {
    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    /*
    IMPORTANT

    |
    | `Rc<Mutex<i32>>` cannot be sent between threads safely
    | the trait `Send` is not implemented for `Rc<Mutex<i32>>`
    |
    */
    for _ in 0 .. 10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
 */

use std::sync::Arc;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    /*
    IMPORTANT

    |
    | `Rc<Mutex<i32>>` cannot be sent between threads safely
    | the trait `Send` is not implemented for `Rc<Mutex<i32>>`
    |
    */
    for _ in 0 .. 10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}