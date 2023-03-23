/*

IMPORTANT

You can take five actions in unsafe Rust that you can’t in safe Rust, which we
call unsafe superpowers. Those superpowers include the ability to:

    1. Dereference a raw pointer
    2. Call an unsafe function or method
    3. Access or modify a mutable static variable
    4. Implement an unsafe trait
    5. Access fields of unions

 */

#[test]
fn main1() {
    let mut num = 5;

    // "Notice that we don’t include the unsafe keyword in this code. We can
    // create raw pointers in safe code; we just can’t dereference raw pointers
    // outside an unsafe block, as you’ll see in a bit."
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    println!("r1 is: {:?}", r1);
    println!("r2 is: {:?}", r2);

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

#[test]
fn main2() {
    unsafe fn dangerous() {}

    // Calling the below without the unsafe block results in
    //
    // | this operation is unsafe and requires an unsafe function or block
    //
    //dangerous();

    // The unsafe keyword in this context indicates the function has
    // requirements we need to uphold when we call this function, because Rust
    // can’t guarantee we’ve met these requirements.
    //
    // By calling an unsafe function within an unsafe block, we’re saying that
    // we’ve read this function’s documentation and take responsibility for
    // upholding the function’s contracts.
    unsafe {
        dangerous();
    }
}

#[test]
fn main3() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    // Equivalent to the above
    //let r = &mut v[..];
    let r = v.as_mut_slice();

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

/* 
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    // The below fails with
    // | 
    // |  cannot borrow `*values` as mutable more than once at a time
    // |  second mutable borrow occurs here
    // | 
    (&mut values[..mid], &mut values[mid..])
}

IMPORTANT

//
// Rust’s borrow checker can’t understand that we’re borrowing different parts
// of the slice; it only knows that we’re borrowing from the same slice twice.
//
// Borrowing different parts of a slice is *fundamentally okay* because the two
// slices aren’t overlapping, but Rust isn’t smart enough to know this.
//
 */

use std::{slice, thread::JoinHandle};

//
// Note that we don’t need to mark the resulting split_at_mut function as
// unsafe, and we can call this function from safe Rust.
//
// We’ve created a safe abstraction to the unsafe code with an implementation of
// the function that uses unsafe code in a safe way, because it creates only valid
// pointers from the data this function has access to.
//
fn split_at_mut<'a>(values : &'a mut [i32], mid : usize) -> (&'a mut [i32], &'a mut [i32]) {
    let len = values.len();
    // `as_mut_ptr` returns a raw pointer `*mut T`, different from Rust's safe default `&mut`.
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // The function slice::from_raw_parts_mut is unsafe because it takes
            // a raw pointer and must trust that this pointer is valid.
            slice::from_raw_parts_mut(ptr, mid),
            // The add method on raw pointers is also unsafe, because it must
            // trust that the offset location is also a valid pointer.
            slice::from_raw_parts_mut(ptr.add(mid), len - mid )
        )
    }
}

#[test]
fn main5() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    // Equivalent to the above
    //let r = &mut v[..];
    let r = v.as_mut_slice();

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

#[test]
/// Will error due to improper use of `unsafe`.
fn main4() {
    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };
    println!("{:?}", values);
}

extern "C" {
    fn abs(input: i32) -> i32;
}

#[test]
fn main6() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

//
//
//

// Example usage of a static variable
static HELLO_WORLD: &str = "Hello, world!";

#[test]
fn main7() {
    println!("name is: {}", HELLO_WORLD);
}

/*

IMPORTANT

    |
    | A subtle difference between constants and immutable static variables is that
    | values in a static variable have a fixed address in memory.
    |
    | Using the value will always access the same data.
    |
    | Constants, on the other hand, are allowed to duplicate their data whenever they’re used.
    |
    | Another difference is that static variables can be mutable. Accessing and modifying mutable static
    | variables is unsafe.
    |

*/

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

/*

IMPORTANT

    |
    | With mutable data that is globally accessible, it’s difficult to ensure there
    | are no data races, which is why Rust considers mutable static variables to be
    | unsafe.
    |
    | Where possible, it’s preferable to use the concurrency techniques and
    | thread-safe smart pointers we discussed in Chapter 16 so the compiler checks
    | that data accessed from different threads is done safely.
    |

 */

fn main() {
    use std::thread;

    add_to_count(1);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    let mut handles : Vec<JoinHandle<()>> = vec![];

    for i in 1..=10 {
        let id = thread::spawn(move || {
            for j in 1..=10 {
                unsafe {
                    add_to_count(1);
                    println!("I am thread {} on iteration {}, and I see {} on COUNTER", i.to_owned(), j, COUNTER);
                }
            }
        });
        handles.push(id);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

//
// Implementing an Unsafe Trait
//

/*

IMPORTANT

    |
    | If we implement a type that contains a type that is not Send or Sync, such as
    | raw pointers, and we want to mark that type as Send or Sync, we must use
    | unsafe.
    |
    | Rust can’t verify that our type upholds the guarantees that it can be
    | safely sent across threads or accessed from multiple threads; therefore, we
    | need to do those checks manually and indicate as such with unsafe.
    |

 */

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}