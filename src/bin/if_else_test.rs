#![feature(auto_traits, negative_impls)]

use cotaint::taint_block;
use cotaint::taint_block_return;

// Using an if-else statement is supported, this is expected to print out "innocent taint sanitized"
fn main() {
    // If-else supported
    let mut my_var = taint_block_return!({ create_taint(str::to_string("innocent")) });
    taint_block!({
        if true {
            std::string::String::push_str(extract_taint_mut_ref(&mut my_var), " taint");
        } else {
            let x = 1;
            let y = 2;
            let _z = x + y;
        }
    });

    let sanitized = my_var.sanitize(|taint| {
        let ret = taint + " sanitized";
        ret
    });
    println!("{sanitized}");
}
