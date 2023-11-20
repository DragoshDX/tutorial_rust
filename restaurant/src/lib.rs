mod front_of_house;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please.", meal.toast);
}

pub use crate::front_of_house::hosting;

pub fn seat_at_restaurant() {
    hosting::add_to_waitlist();
}

pub fn dine_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order1 = back_of_house::Appetizer::Salad;
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Self {
            return Self {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            };
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();

        super::deliver_order()
    }

    fn cook_order() {}
}

fn deliver_order() {}
