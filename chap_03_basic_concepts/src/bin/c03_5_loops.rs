/* fn main() {
    loop {
        println!("again!");
    }
}
 */

/* 
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
 */

/* 
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
 */

/* 
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
 */

/* 
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
 */

/* 
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
 */

/* 
fn main() {
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
 */

/* 
fn main() {
    let mut x = 0;
    'a: loop {
        x += 1;
        'b: loop {
            if x > 10 {
                continue 'a;
            } else {
                break 'b;
            }
        }
        break;
    }

    println!("The value of x is {x}");
}
 */

fn fib(n : u32) -> u32 {
    let mut a = 1;
    let mut b = 1;

    if n == 0 {
        return 0
    } else if n == 1 {
        return a
    } else if n == 2 {
        return b
    }

    let mut n_ = n;

    while n_ > 2 {
        let a_ = a;
        a = b;
        b = a_ + b;
        n_ -= 1;
    }

    b
}

fn main() {
    for i in 0..=10 {
        println!("fib({}) is {}", i, fib(i));
    }
}
