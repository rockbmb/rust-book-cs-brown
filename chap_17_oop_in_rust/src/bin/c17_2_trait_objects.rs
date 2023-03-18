use chap_17_oop_in_rust::{Button, Draw, Screen};

// This type was implemented in the binary crate, outside of the library's
// module defining `Draw`, to simulate a user creating a new type not part
// of the original crate or package, and still wishing to integrate the user
// defined type with other implementors of the trait.
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

/*
This will error with

 | the trait bound `String: Draw` is not satisfied
 | the following other types implement trait `Draw`:
 |   Button
 |   SelectBox
 | required for the cast from `String` to the object type `dyn Draw`

The compiler will still prevent trait objects from being created based on
types that do not implement the required behavior - at compile time!

fn main() {
    let screen = Screen {
        components: vec![Box::new(String::from("Hi"))],
    };

    screen.run();
}
*/


/*

IMPORTANT

 | The advantage of using trait objects and Rust’s type system to write code similar
 | to code using duck typing is that we never have to check whether a value
 | implements a particular method at runtime or worry about getting errors if a value
 | doesn’t implement a method but we call it anyway. Rust won’t compile our code if the
 | values don’t implement the traits that the trait objects need.

*/
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                    width : 75,
                    height : 10,
                    options : vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
            }),
            Box::new(Button {
                width : 50,
                height : 10,
                label : String::from("OK"),
            }),
        ],
    };

    screen.run();
}

/*

IMPORTANT

 |
 | Recall in the “Performance of Code Using Generics” section in Chapter 10 our
 | discussion on the monomorphization process performed by the compiler when we
 | use trait bounds on generics: the compiler generates nongeneric
 | implementations of functions and methods for each concrete type that we use
 | in place of a generic type parameter.
 |
 | The code that results from monomorphization is doing static dispatch, which is
 | when the compiler knows what method you’re calling at compile time. 
 |
 | When we use trait objects, Rust must use dynamic dispatch. The compiler
 | doesn’t know all the types that might be used with the code that’s using trait
 | objects, so it doesn’t know which method implemented on which type to call.
 |
 | Instead, at runtime, Rust uses the pointers inside the trait object to know
 | which method to call.
 |
 | This lookup incurs a runtime cost that doesn’t occur with static dispatch.
 |
 | Dynamic dispatch also prevents the compiler from choosing to inline a method’s
 | code, which in turn prevents some optimizations.
 */