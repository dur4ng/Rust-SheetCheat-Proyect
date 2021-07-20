
// 4. Create a tests/ directory and an integration test file tests/more_tests.rs
// Inside that file, create a test function that verifies:
// - sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)) returns 4
//
// `cargo test` should run your more_tests.rs file and pass

// import the libraries and methos that you will test
use testing::{splish, sploosh} 

#[test]
fn integration_test() {
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}