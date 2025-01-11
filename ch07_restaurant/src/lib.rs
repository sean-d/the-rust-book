/*
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}
*/

/*
mod back_of_the_house {
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        super::super_fuction();
    }

    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("peaches"),
            }
        }
    }
}
*/

mod back_of_the_house;
mod front_of_the_house;

pub fn eat_at_restaurant() {
    front_of_the_house::hosting::add_to_waitlist();
    front_of_the_house::hosting::seat_at_table();
    back_of_the_house::Breakfast::summer("sourdough");
}
