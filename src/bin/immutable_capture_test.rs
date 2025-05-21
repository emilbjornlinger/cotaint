#![feature(auto_traits, negative_impls)]

use cotaint::taint_block_return;

// Capturing immutable untainted variables is supported, this is expected to print out "innocent taint
// sanitized"
fn main() {
    let my_immutable_var = "innocent".to_string();
    let my_taint = taint_block_return!({ create_taint(my_immutable_var + " taint") });

    let sanitized = my_taint.sanitize(|taint| {
        let ret = taint + " sanitized";
        ret
    });

    println!("{}", sanitized);
}
