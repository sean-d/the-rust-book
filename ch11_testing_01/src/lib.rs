#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("guess value must be 1 or more.");
        } else if value > 100 {
            panic!("guess value must be 100 or less.");
        }

        Guess { value: value }
    }
}
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
/*
tests live in a tests module, everything else lives in the parent module...super.

so we need to give access to the stuff outside of the tests module....

we do that via use....and to bring it all in:
use super::*;

note: anything passed into assert_eq or assert_ne have to implement the traits: partial_eq and debug

most of the stdlib have this but custom things we make need to have them added

#[test]: this is a function that runs when cargo test is run
#[should_panic]: the function being tested should panic. if a function is set with this and it doesn't panic, the test fails

can supply the expected message from the panic in the should_panic and that will be useful to determine if an expected panic is legit/as expected.
*/
mod tests {

    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 100,
            height: 200,
        };

        let smaller = Rectangle {
            width: 10,
            height: 20,
        };

        assert_eq!(larger.can_hold(&smaller), true);
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let smaller = Rectangle {
            width: 10,
            height: 20,
        };

        let larger = Rectangle {
            width: 100,
            height: 200,
        };

        assert_eq!(smaller.can_hold(&larger), false);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("me");
        assert!(result.contains("me"));
    }

    #[test]
    fn greeting_contains_name_fails() {
        let result = greeting("failure");
        assert!(
            result.contains("me"),
            "greeting did not containt the expected name. the value was {result}"
        );
    }

    #[test]
    fn legit_guess() {
        Guess::new(50);
    }

    #[test]
    #[should_panic]
    fn less_than_1() {
        Guess::new(0);
    }

    #[test]
    #[should_panic(expected = "guess value must be 100 or less.")]
    fn greater_than_100() {
        Guess::new(101);
    }
}
