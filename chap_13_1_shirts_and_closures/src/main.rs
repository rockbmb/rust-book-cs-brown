#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main1() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

//
//
//

use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

fn main2() {
    // Possible syntax of fn/closure definitions
    fn  add_one_v1   (x: u32) -> u32 { x + 1 };
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x : u32|             { x + 1 };
    let add_one_v4 = |x : u32|               x + 1  ;

    let example_closure = |x| x;

    //let n = example_closure(5);
    let s = example_closure(String::from("hello"));

    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

//
//
//

fn main3() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // `println!` only requires an immutable borrow, so the compiler
    // will give this closure the appropriate type.
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn main4() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

/*
IMPORTANT

 | Between the closure definition and the closure call, an immutable borrow to print
 | isn’t allowed because no other borrows are allowed when there’s a mutable borrow.
*/

    let mut borrows_mutably = || list.push(7);

    // Will error, see above.
    //println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn main5() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

/*
IMPORTANT

 | The new thread might finish before the rest of the main thread finishes, or the main thread
 | might finish first.
 |
 | If the main thread maintained ownership of list but ended before the new thread did and dropped
 | list, the immutable reference in the thread would be invalid.
 |
 | Therefore, the compiler **requires** that list be moved into the closure given to the new thread so
 | the reference will be valid. 
 */

    // Uncommenting `move` results in
    // | "closure may outlive the current function, but it borrows `list`, which is owned by the current function"
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

        // The below line cannot be enabled, as the vector has been `move`d into the
        // thread, and is therefore no longer available in this execution path.
        //println!("After defining closure: {:?}", list);
}

//
//
//

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main6() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);
}

/* 
fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        // | cannot move out of `value`, a captured variable in an `FnMut` closure
        // | move occurs because `value` has type `String`, which does not implement
        // | the `Copy` trait
        sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list);
}
 */


fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

//
//
//

/*
 | hidden type for `impl Fn() -> String` captures lifetime that does not appear in bounds
 |
 | hidden type `[closure@src/main.rs:234:5: 234:12]` captures the anonymous lifetime defined here
 */

/* 
fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
    move || s_ref.to_string()
}
 */

//              vvvv         vv                             vvvv                
fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()
}

// Making use of lifetime elision rules to shorten function type.
fn make_a_cloner2(s_ref: &str) -> impl Fn() -> String + '_ {
    || s_ref.to_string()
}

/*
fn main() {
    let s_own = String::from("Hello world");
    let cloner = make_a_cloner2(&s_own);
    drop(s_own);
    cloner();
}
*/

fn for_each_mut<T, F: ___(&mut T)>(v: &mut Vec<T>, mut f: F) {
    for x in v.iter_mut() {
        //f(x);
    }
}