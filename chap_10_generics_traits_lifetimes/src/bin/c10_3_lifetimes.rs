/// Quick ownership refresher

/*
fn main1() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
*/

/* 
/// With lifetimes explicited:
fn main() {
    let r: &i32;          // ---------+-- 'a
                          //          |
    {                     //          |
        let x: i32 = 5;   // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+
 */

use std::fmt::Display;

fn main2() {
    let x: i32 = 5;       // ----------+-- 'b
                          //           |
    let r: &i32 = &x;     // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

///
///
///

/* 
/// The error gotten here explains why this function requires explicit
/// lifetime annotations.
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
 */

/* 
 IMPORTANT:

 | One lifetime annotation by itself doesn’t have much meaning, because the
 | annotations are meant to tell Rust how generic lifetime parameters of multiple
 | references relate to each other.

 From chapter [10.3](https://rust-book.cs.brown.edu/ch10-03-lifetime-syntax.html#lifetime-annotation-syntax)
 */

/// Fixed with lifetimes: the returned reference will be valid for as long as the
/// shortest-lived parameter's reference's lifetime
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main3() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn main4() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}

/* 
/// Why the below fails: `result` lives longer than the shortest-lived
/// reference, which is `string2`.
fn main4() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
 */

///
///
///

/// Parameters that will never be returned don't need a lifetime.
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

/* 
/// Fails because the return value's lifetime is not related to that of either
/// argument.
/// Lifetime specifiers will never enable dangling references.
fn longest3<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
 */

/* 
 IMPORTANT:

 | Ultimately, lifetime syntax is about connecting the lifetimes of various parameters
 | and return values of functions.
 |
 | Once they’re connected, Rust has enough information to allow memory-safe operations
 | and disallow operations that would create dangling pointers or otherwise violate
 | memory safety.

 */

 struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

/*
| The compiler uses three rules to figure out the lifetimes of the references when there
| aren’t explicit annotations.
| * The first rule is that the compiler assigns a different lifetime parameter to each
|   lifetime in each input type.
| * The second rule is that, if there is exactly one input lifetime parameter, that
|   lifetime is assigned to all output lifetime parameters.
| * The third rule is that, if there are multiple input lifetime parameters, but one of
|   them is &self or &mut self because this is a method, the lifetime of self is assigned
|   to all output lifetime parameters.
 */

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    let mut elems = elems.to_vec();
    elems.sort();
    let t = &elems[n];
    return t.clone();
}

fn find_nth2<T: Ord + Clone>(elems: &[T], n: usize) -> T {
    let mut elem_refs: Vec<&T> = elems.iter().collect();
    elem_refs.sort();
    let t = elem_refs[n];
    return t.clone();
}