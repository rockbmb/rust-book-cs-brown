/*

IMPORTANT

    |
    | In other words, when a trait has a generic parameter, it can be implemented
    | for a type multiple times, changing the concrete types of the generic type
    | parameters each time.
    |
    | Associated types also become part of the trait’s contract: implementors of
    | the trait must provide a type to stand in for the associated type
    | placeholder.
    |
    | Associated types often have a name that describes how the type
    | will be used, and documenting the associated type in the API documentation is
    | good practice.
    |

 */

use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn main1() {
     assert_eq!(
         Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
         Point { x: 3, y: 3 }
     );
}

struct Millimeters(u32);
struct Millimeters2(u32);
struct Meters(u32);

/*

IMPORTANT

//
// If you want to add a type parameter to an existing trait, you can give it a
// default to allow extension of the functionality of the trait without breaking
// the existing implementation code.
//

 */

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

impl Add<Meters> for Millimeters2 {
    type Output = Meters;

    fn add(self, other: Meters) -> Meters {
        Meters((self.0 * 1000) + other.0)
    }
}

impl Add for Meters {
    type Output = Millimeters;

    fn add(self, rhs: Self) -> Self::Output {
        Millimeters((self.0 + rhs.0) * 1000)
    }
}

//
// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
//

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

#[test]
fn main2() {
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    // Syntax from the section below
    <Human as Pilot>::fly(&person);
    <Human as Wizard>::fly(&person);
}

//
//
//

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

#[test]
fn main3() {
    // Not the desired output, as `baby_name` from `Animal` is what is wanted.
    println!("A baby dog is called a {}", Dog::baby_name());

    // Will cause an error, as the compilar does not know which type implementing
    // `Animal` to use.
    //println!("A baby dog is called a {}", Animal::baby_name());

    // Solution:
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    /*
    
    IMPORTANT

    In general, fully qualified syntax is defined as follows:

    `<Type as Trait>::function(receiver_if_method, next_arg, ...);`

     */
}

//
//
//

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

//
// `Point` doesn't implement `std::fmt::Display`
// the trait `std::fmt::Display` is not implemented for `Point`
// in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
//
//impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

#[test]
fn main4() {
    let p = Point { x: 1, y: 3 };
    p.outline_print();
}

//
//
//

/*

IMPORTANT

    |
    | In Chapter 10 in the “Implementing a Trait on a Type” section, we mentioned
    | the orphan rule that states we’re **only allowed to implement a trait** on a type
    | if **either:
    | * the trait or
    | * the type are local to our crate.
    |
    | It’s possible to get around this restriction using the newtype pattern, which
    | involves creating a new type in a tuple struct. (We covered tuple structs in the
    | “Using Tuple Structs without Named Fields to Create Different Types” section of
    | Chapter 5.)
    |
    | The tuple struct will have one field and be a thin wrapper around the
    | type we want to implement a trait for. Then the wrapper type is local to our
    | crate, and we can implement the trait on the wrapper.
    |
    | Newtype is a term that originates from the Haskell programming language.
    |
    | There is no runtime
    | performance penalty for using this pattern, and the wrapper type is elided at
    | compile time.
    |
 */


/// "As an example, let’s say we want to implement Display on Vec<T>, which the
/// orphan rule prevents us from doing directly because the Display trait and
/// the Vec<T> type are defined outside our crate."
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

/*

    |
    | * If we wanted the new type to have every method the inner type has,
    |   implementing the Deref trait (discussed in Chapter 15 in the “Treating Smart
    |   Pointers Like Regular References with the Deref Trait” section) on the
    |   Wrapper to return the inner type would be a solution.
    |
    | * If we don’t want the Wrapper type to have all the methods of the inner
    |   type—for example, to restrict the Wrapper type’s behavior—we would have to
    |   implement just the methods we do want manually.
    |

 */