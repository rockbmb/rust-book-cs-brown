#[test]
// Type synonyms in Rust work much the way they do in Haskell.
// One becomes a synonym of the other, and aech can be used interchangeably.
fn main1() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
}

//
//
//

#[test]
fn main2() {
    let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    fn takes_long_type(f: Box<dyn Fn() + Send + 'static>) {
        // --snip--
    }
    fn returns_long_type() -> Box<dyn Fn() + Send + 'static> {
        // --snip--
        Box::new(|| ())
    }

    // To avoid all the repetitive and verbose types above, a synonym can be defined.
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type2(f: Thunk) {
        // --snip--
    }
    fn returns_long_type2() -> Thunk {
        // --snip--
        Box::new(|| ())
    }
}

//
//
//

use std::fmt;
use std::io::Error;

// IO errors are described in `std::io::Error`, and most IO functions
// use in in a `Result`, as can be seen below.
//
// `type Result<T> = std::result::Result<T, std::io::Error>;`
//
// Thus a synonym can be created to minimize repetition.
pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    fn flush(&mut self) -> Result<(), Error>;

    fn write_all(&mut self, buf: &[u8]) -> Result<(), Error>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Error>;
}

//
// Because it’s an alias, it’s just another Result<T, E>, which means we can use
// any methods that work on Result<T, E> with it, as well as special syntax like
// the ? operator.
//
type Result2<T> = std::result::Result<T, std::io::Error>;
pub trait Write2 {
    fn write(&mut self, buf: &[u8]) -> Result2<usize>;
    fn flush(&mut self) -> Result2<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result2<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result2<()>;
}

// This code is read as “the function bar returns never.” Functions that return
// never are called diverging functions.
fn bar() -> ! {loop {continue}}

//
// The formal way of describing this behavior is that expressions of type ! can
// be coerced into any other type.
//
// `!` is used in `match` arms, with the `continue` keyword, and `loop`.

struct MyOpt<T>(Option<T>);

//
// Rust sees that val has the type T and panic! has the type !, so the result of
// the overall match expression is T.
//
// This code works because panic! doesn’t produce a value; it ends the program.
// In the None case, we won’t be returning a value from unwrap, so this code is
// valid.
//
impl<T> MyOpt<T> {
    pub fn unwrap(self) -> T {
        match self.0 {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}

// Same below
#[test]
fn main3() {
    print!("forever ");
    let bool = true;

    //
    // Here, the loop never ends, so ! is the value of the expression.
    //
    // However, this wouldn’t be true if we included a break, because the loop
    // would terminate when it got to the break.
    //
    loop {
        print!("and ever ");
        if !bool {
            break
        }
    }

    return;
}

//
// Rust needs to know how much memory to allocate for any value of a particular
// type, and all values of a type must use the same amount of memory.
//
// If Rust allowed us to write this code, these two str values would need to
// take up the same amount of space.
//
// But they have different lengths [...]
//
// This is why it’s not possible to create a variable holding a dynamically
// sized type.
/*
fn main() {
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
}
*/

/*

IMPORTANT

// So although a &T is a single value that stores the memory address of where
// the T is located, a &str is two values:
// * the address of the str, and
// * its length.
//
// As such, we can know the size of a &str value at compile time: it’s twice the
// length of a usize. That is, we always know the size of a &str, no matter how
// long the string it refers to is.
//
// In general, this is the way in which dynamically sized types are used in
// Rust: they have an extra bit of metadata that stores the size of the dynamic
// information.
//
// The **golden rule** of dynamically sized types is that **we must always put
// values of dynamically sized types behind a pointer** of some kind.

 */

//
// To work with DSTs, Rust provides the Sized trait to determine whether or not
// a type’s size is known at compile time. This trait is automatically
// implemented for everything whose size is known at compile time.
//
// Because of this, the below
fn generic1<T>(t: T) {
    // --snip--
}

// Becomes
fn generic2<T: Sized>(t: T) {
    // --snip--
}

// In the early stages of compilation.

/// IMPORTANT
///
/// By default, generic functions **will work __only__ on types that have a known
/// size at compile time**. However, you can use the following special syntax to
/// relax this restriction:
fn generic<T: ?Sized>(t: &T) {
    //
    // A trait bound on ?Sized means “T may or may not be Sized” and this
    // notation overrides the default that generic types must have a known size
    // at compile time.
    //
    // The ?Trait syntax with this meaning is only available for Sized, not any
    // other traits.
    //
}

//
// From the subchapter quiz

fn is_equal<T: ?Sized + Eq>(t1: &T, t2: &T) -> bool {
    t1 == t2
}

fn main() {
    println!("{:?}", is_equal("Hello", "world"));
}
