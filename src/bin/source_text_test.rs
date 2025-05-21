#![feature(auto_traits, negative_impls)]

use std::io;

// We can use the source!() macro to automatically wrap the result of an expression in a tainted
// variable
fn main() {
    println!("Write user input ('a' is a malicious characters):");
    let mut buffer = String::new();
    let my_taint = taint_macros::source!({io::stdin().read_line(&mut buffer).unwrap(); buffer});

    let sanitized = my_taint.sanitize(|mut taint| {
        taint.retain(|ch| ch != 'a');
        taint
    });

    println!("{}", sanitized);
}
