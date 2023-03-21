fn main1() {
    let x = 4;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

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

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
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
    }
}