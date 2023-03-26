/*

IMPORTANT

    A function signature must declare the number and type of parameters the
    function has.

    Macros, on the other hand, can take a variable number of
    parameters: we can call println!("hello") with one argument or
    println!("hello {}", name) with two arguments.
    
     Also, macros are expanded before the compiler interprets the meaning of
    the code, so a macro can, for example, implement a trait on a given type.

    A function can’t, because it gets called at runtime and a trait needs to
    be implemented at compile time.

 */

/// An example macro, similar to `vec!`. Some notes from chapter 19.5:
/// * The #[macro_export] annotation indicates that this macro should be made
///     available whenever the crate in which the macro is defined is brought into
///     scope.
#[macro_export]
macro_rules! myvec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

/// Calling
///
/// `myvec![1, 2, 3];`
///
/// will result in the following Rust code:
/// ```
/// let mut temp_vec = Vec::new();
/// temp_vec.push(1);
/// temp_vec.push(2);
/// temp_vec.push(3);
/// temp_vec
/// }
/// ```
fn main() {
    let v = myvec![1, 2, 3];
}

// Attribute-like macros are similar to custom derive macros, but instead of
// generating code for the derive attribute, they allow you to create new
// attributes.
//
// They’re also more flexible: derive only works for structs and enums;
// attributes can be applied to other items as well, such as functions.

/* 
#[route(GET, "/")]
fn index() {}
 */

// This #[route] attribute would be defined by the framework as a procedural
// macro. The signature of the macro definition function would look like this:

/*
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
*/

// Here, we have two parameters of type TokenStream.
//
// * The first is for the contents of the attribute: the GET, "/" part.
//
// * The second is the body of the item the attribute is attached to: in this case,
//   fn index() {} and the rest of the function’s body.

//
// Function-like macros
//

// "Function-like macros define macros that look like function calls.
//
// Similarly to macro_rules! macros, they’re *more flexible than functions*; for
// example, they can take an *unknown number of arguments*.
//
// However, macro_rules! macros can be defined *only* using the match-like syntax
// we discussed in the section “Declarative Macros with macro_rules! for General
// Metaprogramming” earlier.
//
// Function-like macros take a TokenStream parameter and their definition
// manipulates that TokenStream using Rust code as the other two types of
// procedural macros do."
//
// An example from the chapter:

// `let sql = sql!(SELECT * FROM posts WHERE id=1);`

// The sql! macro would be defined like this:

/*
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
*/

// This definition is similar to the custom derive macro’s signature: we receive
// the tokens that are inside the parentheses and return the code we wanted to
// generate.

//
// Summary
//

/*
The term macro refers to a family of features in Rust: declarative macros with
macro_rules! and three kinds of procedural macros:

    * Custom #[derive] macros that specify code added with the derive attribute used
      on structs and enums
    * Attribute-like macros that define custom attributes usable on any item
    * Function-like macros that look like function calls but operate on the tokens
      specified as their argument
*/