/* 
fn main() {
    let x = Box::new(0);

    let y = Box::new(&x);

    let z = ***y;

    let a : u32 = 4;
    let b = &a;

    println!("x: {x}; y: {y}; z:{z}; a: {a}; b: {b}");
    }
 */

/* 
 fn main() {
    let mut s = String::from("hello");
    let s2 = &s;
    let s3 = &mut s;
    s3.push_str(" world");
    println!("{s2}");
  }
*/

/* fn incr(n: &mut i32) {
    *n += 1 
}

fn main() {
    let mut n = 1;
    incr(&mut n);
    println!("{n}");
} */


/* 
// Must reallocate string, bad.
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() {
            dst.push(s.clone());
        }
    }
} */

/* 
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
    let to_add: Vec<String> = 
        src.iter().filter(|s| s.len() > largest.len()).cloned().collect();
    dst.extend(to_add);
} */

/*
fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len();
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}
 */

fn main() {
    let first = String::from("Ferris");
    let full = add_suffix(first);
    let a = Box::new([0; 1_0]);
    let b = a;

    println!("{full}");
    //println!("{}", a[1]);
}


fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

#[derive(Debug)]
struct B {
    b: u8
}

#[derive(Debug)]
struct A {
    a: u8,
    b: B
}

/// From "Move by Example" book,
/// https://move-book.com/advanced-topics/ownership-and-references.html#borrow-checking
#[test]
fn test() {
    let b = B {b: 9};
    let mut a = A { a: 10, b};

    // --->
    let x = &mut a;
    // <---
    // cannot borrow `a.b` as mutable more than once at a time
    // second mutable borrow occurs here
    let y = &mut a.b;
    // The below fails with the error above, if uncommented
    // println!("{:?}", x);
    println!("{:?}", y);
}