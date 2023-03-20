/// Finding the maximum of a list of numbers
fn main1() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
    assert_eq!(*largest, 100);
}

///
///
///

/// Doing the same thing for two lists
fn main2() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

///
///
///

fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// Extracting the repeated code into a function.
fn main3() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

///
///
///

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// Generalizing `largest` over two different types.
fn main4() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);
    assert_eq!(*result, 100);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
    assert_eq!(*result, 'y');
}

///
///
///

/* 
fn largest2<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // The below line requires that `T` be restricted with the
        // `PartialOrd` trait.
        if item > largest {
            largest = item;
        }
    }

    largest
}
*/

fn largest2<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // The below line requires that `T` be restricted with the
        // `PartialOrd` trait.
        if item > largest {
            largest = item;
        }
    }

    largest
}

/// Generalizing `largest` over any type.
fn main5() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest2(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest2(&char_list);
    println!("The largest char is {}", result);
}

///
///
///

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    // Different types, `T` is required on both fields.
    //let wont_work = Point { x: 5, y: 4.0 };

    let both_integer = Point2 { x: 5, y: 10 };
    let will_work = Point2 { x: 5, y: 4.0 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    println!("p.x = {}", integer.x());

    let p3 = will_work.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}