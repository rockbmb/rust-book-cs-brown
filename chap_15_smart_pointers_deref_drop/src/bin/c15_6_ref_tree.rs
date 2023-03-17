use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node1 {
    value: i32,
    children: RefCell<Vec<Rc<Node1>>>,
}

fn main1() {
    let leaf = Rc::new(Node1 {
        value: 3,
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node1 {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
}

/// In order for a tree's child nodes to point to their parents, while avoiding
/// the reference cycle problem `Node1` would have if it were tried with it,
/// it is necessary to use weak references.
#[derive(Debug)]
struct Node2 {
    value: i32,
    parent: RefCell<Weak<Node2>>,
    children: RefCell<Vec<Rc<Node2>>>,
}

fn main2() {
    let leaf = Rc::new(Node2 {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node2 {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

//
//
//

fn main3() {
    let leaf = Rc::new(Node2 {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    //^ Should print 1 0

    {
        let branch = Rc::new(Node2 {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        //^ Should print 1 1

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        //^ Should print 2 0
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    //^ Should print 1 0
}

/// Exercise at the end of 15.6
fn main4() {
    let r1_printer =
        |s, r| println!("{}: {} {}", s, Rc::strong_count(r), Rc::weak_count(r));

    let r1 = Rc::new(0);
    r1_printer("FIRST", &r1);

    let r4 = {
        let r2 = Rc::clone(&r1);
        r1_printer("SECOND", &r1);
        let x = Rc::downgrade(&r2);
        r1_printer("SECOND PRIME", &r1);
        x
    };
    r1_printer("THIRD", &r1);
    let r5 = Rc::clone(&r1);
    r1_printer("FOURTH", &r1);
    let r6 = r4.upgrade();

    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));

}

fn main() {
    let r1_printer =
        |s, r| println!("{}: {} {}", s, Rc::strong_count(r), Rc::weak_count(r));

    let r1 = Rc::new(0);
    r1_printer("FIRST", &r1);

    let r4 = {
        let r2 = Rc::clone(&r1);
        r1_printer("SECOND", &r1);
        let x = Rc::downgrade(&r2);
        r1_printer("SECOND PRIME", &r1);
        x
    };

    // Consider r4 and r5: one created directly from r1, and the other
    // from a strong reference to r1.
    // They both count toward weak references to r1, which means that
    // after the strong reference r5 points to goes out of scope
    // - r2 below -, it will be transferred to what r2 pointed to,
    // which was r1.
    let r5 = {
        let r2 = Rc::clone(&r1);
        r1_printer("THIRD", &r1);
        let x = Rc::downgrade(&r1);
        r1_printer("THIRD PRIME", &r1);
        x
    };
    println!("{} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
}