mod front_of_house;

pub use crate::front_of_house::hosting;

fn deliver_order() {}

pub mod back_of_house {
    pub fn fix_incorrect_order() {
        // cook_order();
        super::deliver_order();
    }

    pub fn request_order(breakfast: Breakfast) {
        cook_order(breakfast);
    }

    fn cook_order(breakfast: Breakfast) {
        println!("Toast used: {:?}\nFruit used: {:?}", breakfast.toast, breakfast.seasonal_fruit);
    }

    // Here we allow a user to specify what toast they want but not what fruit
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Chef can change what fruit is in season/toast
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path, better if we were to move 'eat_at_restaurant' we won't have to change this 
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path, would change if we moved 'eat'
    front_of_house::hosting::add_to_waitlist();

    crate::back_of_house::fix_incorrect_order();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Customer can change mind on bread type
    meal.toast = String::from("Wheat");
    // Customer can't see or edit the fruit
    println!("{:?}", meal.toast);

    crate::back_of_house::request_order(meal);
}