fn main1() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}

fn main2() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

fn main3() {
    let mut x: u32 = 1;
    {
      let mut x = x;
      x += 2;
    }
    println!("{x}");
}

fn main4() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}

fn main5() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}

fn main6() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}

fn main() {
    let message = "The temperature today is:";
    let x = [message; 100];
    println!("{} {}", x[0], x[1]);
  }
