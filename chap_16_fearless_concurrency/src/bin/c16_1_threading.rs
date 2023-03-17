use std::thread;
use std::time::Duration;

fn main1() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn main2() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Uncommenting the below will make the programming sequential
    // instead of concurrent.
    // Also, the borrow checker enforces that each thread can only be joined
    // once, because `join()` demands ownership of the handle.
    //handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    //handle.join().unwrap();
}

fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

/* 
fn main() {
    let v = vec![1, 2, 3];
    /*
    IMPORTANT

    |
    | By telling Rust to move ownership of v to the spawned thread, we’re guaranteeing
    | Rust that the main thread won’t use v anymore.
    |
     */
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // The spawned thread took ownership of anything it captured
    // from its environment, `v` included, via `move`.
    // As such, the main thread can no longer access those values.
    drop(v); // oh no!

    handle.join().unwrap();
}
 */
