/* mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
} */

/* 
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
 */

/*

/// Will be rendered in `back_of_house`.
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
*/

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

pub fn eat_at_restaurant2() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

/// Chapter 7.4

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//use crate::front_of_house::hosting;

pub fn eat_at_restaurant3() {
    hosting::add_to_waitlist();
}

/* 
/// the above `use` will not be visible to the below module,
/// which causes an error.
mod customer {
        pub fn eat_at_restaurant3() {
        hosting::add_to_waitlist();
    }
}
 */

/// Fix 1: move `use` to interior of relevant child module.
mod customer1 {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant3() {
        hosting::add_to_waitlist();
    }
}

/// Fix 2: use `super` and reference parent module's `use`.
mod customer2 {
    use super::hosting;

    pub fn eat_at_restaurant3() {
        hosting::add_to_waitlist();
    }
}

mod front_of_house1;

pub use crate::front_of_house1::hosting;

pub fn eat_at_restaurant4() {
    hosting::add_to_waitlist();
}