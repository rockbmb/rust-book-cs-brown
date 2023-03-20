/*

IMPORTANT

// |
// | Function parameters, let statements, and for loops can only accept irrefutable
// | patterns, because the program cannot do anything meaningful when values don’t
// | match.
// |
// | The if let and while let expressions accept refutable and irrefutable
// | patterns, but the compiler warns against irrefutable patterns because by
// | definition they’re intended to handle possible failure: the functionality of a
// | conditional is in its ability to perform differently depending on success or
// | failure.
// |

*/

fn main() {
    let some_option_value: Option<i32> = None;
    // using a refutable pattern where an irrefutable one is needed!
    //let Some(x) = some_option_value;
    // Fix:
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // using an irrefutable pattern where a refutable one is expected works,
    // but with a compiler warning, as it'll always match, and be thus useless.
    if let x = 5 {
        println!("{}", x);
    };

    /*
    
    IMPORTANT
    // |
    // | For this reason, match arms must use refutable patterns, except for the
    // | last arm, which should match any remaining values with an irrefutable
    // | pattern.
    // |
     */
}