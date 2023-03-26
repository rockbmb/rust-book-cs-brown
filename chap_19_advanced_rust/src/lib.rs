#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// The function that defines a procedural macro takes a TokenStream as an input
// and produces a TokenStream as an output.
//
// The TokenStream type is defined by the proc_macro crate that is included with
// Rust and represents a sequence of tokens.
//
// This is the core of the macro: the source code that the macro is operating on
// makes up the input TokenStream, and the code the macro produces is the output
// TokenStream.
//
// The function also has an attribute attached to it that specifies which kind
// of procedural macro we’re creating. We can have multiple kinds of procedural
// macros in the same crate.
/*
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {}
*/

// A user could implement the trait above as
/*
use hello_macro::HelloMacro;

struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}
*/
// But the intent of the chapter is to do this with macros.