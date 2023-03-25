fn add_one(x: i32) -> i32 {
    x + 1
}

// Functions coerce to the type fn (with a lowercase f), not to be confused with
// the Fn closure trait.
//
// The fn type is called a function pointer.
//
// Passing functions with function pointers will allow you to use functions as
// arguments to other functions.

// Example of passing a function pointer as an argument.
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[test]
fn main1() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

// Unlike closures, fn is a type rather than a trait!
//
// Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce).

fn main() {
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
    // Passing a closure to `map`
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
    // `fn` pointers implement all 3 closure traits, so the below can be done as well.
    // Fully qualified syntax for trait method required.
        list_of_numbers.iter().map(ToString::to_string).collect();

    // Similarly in Haskell with its ADTs and constructors, enum variants can be used as
    //  functions, and passed accordingly as `fn` pointers to and from other functions.
    enum Status {
        Value(u32),
        Stop,
    }

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// Fails with
//
// `return type cannot have an unboxed trait object`
/* 
fn returns_closure1() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
*/

/// One solution, which restricts us to always returning the same type - in the
/// case of this particular return type, not relevant, as only closures and functions
/// implement it, and can be used interchageably as seen below.
pub fn returns_closure2() -> impl Fn(i32) -> i32 {
    let bool = false;

    if bool {
        add_one
    } else {
        |x| x + 1
    }
}

/// Another solution, this time with smart pointers (chapter 15) and dynamically
/// dispatched trait objects (chapter 17).
pub fn returns_closure3() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

//
// From the exercise in the chapter
//

struct Event;


// Question 1
//
// Consider implementing a register function that takes a callback in two ways:
fn register1(cb: fn(Event) -> ()) {}
fn register2<F>(cb: F) where F: Fn(Event) -> () {}

// Which type signature permits register to take the widest variety of arguments?
// Answer: register2.
//
// IMPORTANT
//
// "Closures with environments can be passed to register2, while **only** top-level
// functions (or closures **without environments**) can be passed to register1.""