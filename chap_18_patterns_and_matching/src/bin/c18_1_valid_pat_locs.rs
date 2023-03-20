fn main1() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    // Shadowed variable permitted by use of if let
    // | The shadowed age we want to compare to 30 isn’t valid until the new scope
    // | starts with the curly bracket.
    // |
    // | The downside of using if let expressions is that the compiler doesn’t
    // | check for exhaustiveness, whereas with match expressions it does.
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

//

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

//

    // Won't work, types don't match
    //let (x, y) = (1, 2, 3);
    let (x, y, z) = (1, 2, 3);

    let point = (3, 5);
    print_coordinates(&point);
}