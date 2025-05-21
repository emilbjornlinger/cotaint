#![feature(auto_traits, negative_impls)]

use cotaint::taint_block_return;
use cotaint::taint_block;

// Will not compile - use of methods on safe variable
fn main() {
    taint_block!({
        let my_str = str::to_string("hello world");

        my_str.push_str(" hello again");
    });
}
