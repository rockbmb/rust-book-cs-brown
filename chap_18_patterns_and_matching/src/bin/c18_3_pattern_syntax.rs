fn main1() {
    let x = 4;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

#[test]
fn main2() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Because `match` introduces a new scope, then within it the following
        // occurence of `y` shadows the outer one - but only until the scope ends.
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn main3() {
    let x = 4;

    match x {
        1 | 2 => println!("one or two"),
        3 | 4 | 5 => println!("three or four or five"),
        _ => println!("anything"),
    }
}

fn main4() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
}

//Ranges of `char` values
fn main5() {
    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn main6() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    // less verbose alternative using field shorthand.
    let Point { x, y } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn main7() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Exit,
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeColor2(Color)
}

#[test]
fn main8() {
    //let msg = Message::ChangeColor(0, 160, 255);
    let msg = Message::ChangeColor2(Color::Hsv(0, 0, 0));
    //let msg = Message::ChangeColor2(Color::Hsv(0, 160, 255));

    match msg {
        // For enum variants without any data, like Message::Quit, we can’t
        // destructure the value any further. We can only match on the literal
        // Message::Quit value, and no variables are in that pattern.
        Message::Quit | Message::Exit => {
            println!("The Quit and Exit variants have no data to destructure.")
        }
        Message::Move { x : f, y : g } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                f, g
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor2(Color::Hsv(0, 0, 0)) => println!(
            "Change the color to red 0, green 0, and blue 0"
        ),
        Message::ChangeColor2(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColor2(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
    }

    // Example of nested destructuring
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

//
//
//

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

#[test]
fn main9() {
    foo(3, 4);
}

#[test]
fn main10() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    //

    let numbers = (2, 4, 8, 16, 32);
    // underscore-prefix varaiable names to supress unused warning
    let _numbers2 = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

#[test]
fn main11() {
    let s = Some(String::from("Hello!"));

    // Using `_s` will move the string out of `s`, and the print below will error.
    // Using `_` will not move the string to any value, and `s` can still be used.
    //if let Some(_s) = s {
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}

#[test]
fn main12() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }
    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { z, .. } => println!("z is {}", z),
    }

    match origin {
        Point { x, z, .. } => println!("x + z is {}", x + z),
    }

/* 
    Ambiguous instance of `..` will result in error

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
 */

}

#[test]
fn main13() {
    let num = Some(64);

    match num {
        Some(x) if (50..=100).contains(&x) => println!("Between 50 and 100"),
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}

#[test]
// Repeat of `main2` withouth shadowing
fn main14() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

#[test]
fn main15() {
    let x = Some(4);
    let y = false;

    match x {
        // Combining multiple patterns, ranges and match guards
        Some(4 | 5 | 6) | Some(14..=19) if !y => println!("yes"),
        _ => println!("no"),
    }
}

fn main() {
    enum Message {
        Hello { id: i32, nothing : bool },
    }

    let msg = Message::Hello { id: 27 , nothing : false};

    let bool = true;

    // Mixing match guards, range patterns, multiple patterns, @ bindings,
    // struct destructuring and `..` struct syntax
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
            ..
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12, .. } => {
            println!("Found an id in another range")
        }
        Message::Hello {
            id: id_variable @ (27 | 31 | 37),
            ..
        } if bool => println!("BIG PRIME1: {}", id_variable),
        Message::Hello {
            id: id_variable @ (27 | 31 | 37),
            ..
        } if !bool => println!("BIG PRIME2: {}", id_variable),
        Message::Hello { id: 17 | 19 | 23 , ..} => {
            println!("Small prime, between 17 and 23");
        }
        Message::Hello { id, .. } => println!("Found some other id: {}", id),
    }

    let a = [(0, 1)];

    let [..] = a;
}