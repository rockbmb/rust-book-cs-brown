pub trait Draw {
    fn draw(&self);
}

/*
IMPORTANT

| This works differently from defining a struct that uses a generic type parameter
| with trait bounds. A generic type parameter can only be substituted with one
| concrete type at a time, whereas trait objects allow for multiple concrete types
| to fill in for the trait object at runtime.

If this were to be the definition of the `Screen` struct,
then we would still have polymorphism, but homogeneous collections
at compile time since the `T` would have to be a stable concrete
type throughout the program which implemented `Draw`, and *not*
a variety of possible types which did.

| If you’ll only ever have homogeneous collections, using generics and trait bounds
| is preferable because the definitions will be monomorphized at compile time to
| use the concrete types.

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

*/

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}