fn cook_order() {}
fn fix_incorrect_order() {}

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
