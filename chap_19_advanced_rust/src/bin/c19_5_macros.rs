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