/*

/*
debug for printing
partialeq for equality comparison
clone to make clones
cope to make copies
*/
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

/*
Inventory method giveaway takes an Option<ShirtColor> as an argument for the shit color preference.

returns an Option<ShirtColor> which will be Some or None.

the return uses unwrap_or_else that takes one arg, a closure,
we pass in that closure with zero arguments/params and have it return the result of calling the most_stocked function for the inventory
*/
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    /*
    init the shirt color counters

    iterate over shirts vector in Inventory
    match on the ShirtColor and increment based on result

    determine which counter is higher and return the ShirtColor enum associated with said counter
     */
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

fn main() {
    // establish the store and populate the shirts available
    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    // one pref set, one has no pref. the set one will return the color they want, the other will return the most stocked color
    let user_pref1 = Some(ShirtColor::Red);
    let user_pref2 = None;

    let giveaway1 = store.giveaway(user_pref1);

    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "Giveaway 1: User with pref {:?} gets {:?}",
        user_pref1, giveaway1
    );
    println!(
        "Giveaway 1: User with pref {:?} gets {:?}",
        user_pref2, giveaway2
    );
}
*/
