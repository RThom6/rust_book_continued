mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path, better if we were to move 'eat_at_restaurant' we won't have to change this 
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path, would change if we moved 'eat'
    front_of_house::hosting::add_to_waitlist();
}