#![feature(auto_traits, negative_impls)]

use cotaint::taint_block_return;
use cotaint::taint_block;

// Will not compile - side effect
fn main() {
    let mut my_var = taint_block_return!({ create_taint(str::to_string("innocent")) });
    let mut my_string = "other".to_string();
    taint_block!({
        std::string::String::push_str(extract_taint_mut_ref(&mut my_var), " taint");
        std::string::String::push_str(&mut my_string, " taint");
    });
}
