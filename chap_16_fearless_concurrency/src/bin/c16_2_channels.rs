/*
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
*/

use std::sync::mpsc;
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

fn main() {
    let (tx, rx) = mpsc::channel();
    let threads = 5;

    for i in 0 .. threads {
        let tx1 = tx.clone();
        thread::spawn(move || {
            let nums : Vec<i32> = vec![2, 3, 5, 7, 11];
            let multiples = vec![1, 2, 3, 4, 5];

            let n = nums[i];
            let mults = multiples
                .iter().map(|m| n * m).collect::<Vec<_>>();

            for val in mults {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_millis(250));
            }

            //drop(tx1);
        });
    }

    drop(tx);

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--
}