/*
Attempt 1

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
} */

/*
Attempt 2, tuples

fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
} */

/*
Attempt 3, with struct

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
} */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other : &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            true
        } else {
            false
        }
    }
}

/* fn main() {
    let scale = 1;

    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    dbg!(&rect1);
    //println!("rect1 is {:#?}", rect1);
} */

impl Rectangle {
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn max(self, other: Rectangle) -> Rectangle {
        Rectangle { 
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

fn main1() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect4 = &rect2;
    let max_rect = rect1.max(rect2);

    // If this line is uncommented, then rect4 is still borrowing rect2
    // when the previous line is ran, which is a compile-time error, as Rectangle::max
    // will need to move both rect, and rect2.
    //println!("rect4 is {:#?}", rect4);
}

struct Point {
    x: i32,
    y: i32
  }

  impl Point {
    fn get_x(&mut self) -> &mut i32 {
      &mut self.x
    }
  }

  fn main() {
    let mut p = Point { x: 1, y: 2 };
    let x = p.get_x();
    *x += 1;
    // Because `get_x` mutably borrows all of p, it cannot be used until x goes out of scope.
    // Therefore, the below println won't work.
    //println!("{} {}", *x, p.y);
    println!("{}", *x);
    println!("{}", p.y);
}