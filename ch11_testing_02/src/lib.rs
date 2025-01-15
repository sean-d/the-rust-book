/*
all tests run in their own thread.

cargo test --help
cargo test -- --help

first one shows options for what flags can be used for the cargo test command

second one shows options for what flags can be used for the resulting test binary

why the second one?
say you are editing a file, doing so in parallel could cause problems.
by setting the thread count to 1, that means only one test runs at a time and that means the file modifications won't be problematic.

we can also use a flag to the testing binary to show output. by default, tests that succeed do not print to stdout...that's captured by the tests. but we can tell the tests to allow for printing...

cargo test -- --show-output will print "i got you" from the print_test() test.


we can run a specific test:
cargo test function_name

we can also run a subset of tests with a shared name...
if we have test called: it_works, it_still_works, its_alive, bounce

cargo test it: this will run the first 3 listed above as all start with "it"

when you run that, you see they are part of the tests:: module name. so you can also run tests in a given module...

cargo test tests: will test all test functions in the tests module.

we can ignore tests we don't wish to run.
really_expensive_test takes hours to run. so we only want to run that every so often.

to ignore this test:
add #[ignore] above the function name

to run only the ignored test:
cargo test -- --ignored
*/

// this is used to tell cargo to only compile this code when we run 'cargo test'

// unit tests live in the file they are testing. just what it is. integration tests live in a folder called tests in the project root.
pub fn add_two(a: i32) -> i32 {
    internal_adding(a, 2)
}

fn internal_adding(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_test() {
        assert_eq!(add_two(2), 4);
    }
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }

    fn show_output() -> i32 {
        println!("i got you");
        5
    }

    #[test]
    fn it_works2() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn it_still_works() {
        assert_eq!(1 + 1, 2);
    }
    #[test]
    fn its_alive() {
        assert_eq!("this", "this");
    }

    #[test]
    fn print_test() {
        assert_eq!(show_output(), 5);
    }

    #[test]
    #[ignore]
    fn really_expensive_test() {}
}
