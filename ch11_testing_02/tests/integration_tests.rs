/*
we have to bring ch11_testing_02 into scope because every file in the test directory gets turned into a separate crate by cargo.

every test is annotated with test. there are no modules with the config annotation, #[cfg(test)], because cargo knows that all files in the test directory are tests.

when we run cargo test, the unit tests, integration tests, and doc tests are listed in sections.

to only run our integration_tests:
cargo test --test integration_tests

to create the ability to share stuff between tests, we create a subfolder in tests called "common" and in there a file called "mod.rs". in there we place shared code.

by placing it there, cargo test won't create a crate and test setup for it.

then we just bring that module in and use what's it in.


you cannot test a binary crate with integration tests...so it's common to see a binary crate as a thing wrapper around a library crate....
*/
use ch11_testing_02;

mod common;

#[test]
fn it_adds_two() {
    common::setup(); // bring in from common/mod.rs
    assert_eq!(ch11_testing_02::add_two(2), 4);
}
