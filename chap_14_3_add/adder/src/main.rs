use add_one::internal;
use add_two;
// Even though `rand` is used by `add_one`, a fellow workspace member, we still need
// to declare it in this package in order to use it.
//use rand;

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}!", internal::add_one(num));
    println!("Hello, world! {num} plus two is {}!", add_two::add_two(num));
}
