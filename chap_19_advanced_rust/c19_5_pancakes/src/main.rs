use c19_5_hello_macro::HelloMacro;
use c19_5_hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}