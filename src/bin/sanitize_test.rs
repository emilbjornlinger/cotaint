#![feature(auto_traits, negative_impls)]

use cotaint::taint_block_return;

// Sanitizing tainted variables is supported, this is expected to print out "taint sanitized"
fn main() {
    let my_taint = taint_block_return!({ create_taint(str::to_string("taint")) });

    let sanitized = my_taint.sanitize(|taint| {
        let ret = taint + " sanitized";
        ret
    });

    println!("{}", sanitized);
}
