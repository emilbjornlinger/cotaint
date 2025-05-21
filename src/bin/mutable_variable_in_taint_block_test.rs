#![feature(auto_traits, negative_impls)]

use cotaint::taint_block_return;

// Using mutable untainted variables inside taint blocks is supported, this is expected to print out "hello world"
fn main() {
    let my_taint = taint_block_return!({
        let mut my_str = str::to_string("hello ");

        std::string::String::push_str(&mut my_str, "world");
        create_taint(my_str)
    });

    println!("{}", my_taint.sanitize(|taint| taint));
}
