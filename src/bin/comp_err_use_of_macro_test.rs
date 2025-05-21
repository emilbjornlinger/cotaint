#![feature(auto_traits, negative_impls)]

use cotaint::taint_block_return;
use cotaint::taint_block;

// Will not compile - use of macro
fn main() {
    taint_block!({
        let x = vec![1, 2, 3];
    });
}
