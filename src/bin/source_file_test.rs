#![feature(auto_traits, negative_impls)]

use std::fs;

// We can use the source!() macro to automatically wrap the result of an expression in a tainted
// variable
fn main() {
    let my_file_taint = taint_macros::source!(fs::read_to_string("src/input/test_input.txt").unwrap());

    println!(
        "{}",
        my_file_taint.sanitize(|a| format!("{a} sanitized"))
    );
}
