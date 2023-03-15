use std::ops::Deref;

fn main() {
    let x = 5;
    let w = MyBox::new(x);
    let y = &x;
    let z = Box::new(x);

    assert_eq!(5, *w);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    //assert_eq!(5, y);
    //assert_eq!(5, z);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // What would be required without deref coercion.
    hello(&(*m)[..]);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//
//
//

fn hello(name: &str) {
    println!("Hello, {name}!");
}
