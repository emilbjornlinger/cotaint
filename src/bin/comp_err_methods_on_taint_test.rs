#![feature(auto_traits, negative_impls)]

use cotaint::taint_block_return;
use cotaint::taint_block;

// Will not compile - use of methods on taint
fn main() {
    let mut my_var = taint_block_return!({ create_taint(str::to_string("innocent")) });
    taint_block!({
        std::string::String::push_str(extract_taint_mut_ref(&mut my_var), " taint");
        extract_taint_mut_ref(&mut my_var).push_str(" taint");
    });
}
