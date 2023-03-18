/*
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
*/

use std::sync::{mpsc, Arc};
use std::thread;

/*
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
}
*/

fn main1() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

/*
fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // Attempt to use a value after it has been sent will not work,
        // because it will have been moved by then
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
*/

use std::time::Duration;

fn main2() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

//
//
//

fn main3() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

//
//
//

// Below is a variable number of threads running and writing to the same
// MPSC channel.
fn main() {
    let (tx, rx) = mpsc::channel();
    let threads = 5;

    // In order to share these vectors between threads, and not have to
    // declare them repeatedly inside the thread's closure, they must be
    // wrapped in an `Arc`.
    let nums = Arc::new(vec![2, 3, 5, 7, 11]);
    let multiples = Arc::new(vec![1, 2, 3, 4, 5]);

    for i in 0 .. threads {
        let tx1 = tx.clone();
        let nums = nums.clone();
        let multiples = multiples.clone();
        thread::spawn(move || {
            let n = nums[i];
            let mults = multiples
                .iter().map(|m| n * m).collect::<Vec<_>>();

            for val in mults {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_millis(250));
            }

            //drop(tx1);
            drop(nums);
            drop(multiples);
        });
    }

    // If the transmitter for the main thread is not dropped here,
    // it will hang even after all children are finished writing.
    // This doesn't seem to be documented anywhere, unsure why it happens.
    drop(tx);

    let mut count = 0;
    for received in rx.iter() {
        print!("Got: {} --- ", received);
        count += 1;
        println!("main thread: message number {count}");
    }

    // --snip--
}