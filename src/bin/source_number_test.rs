#![feature(auto_traits, negative_impls)]

// We can use the source!() macro to automatically wrap the result of an expression in a tainted
// variable
fn main() {
    let my_taint = taint_macros::source!(3);

    let sanitized = my_taint.sanitize(|taint| taint);

    println!("{}", sanitized);
}
