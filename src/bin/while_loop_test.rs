#![feature(auto_traits, negative_impls)]

use cotaint::taint_block;
use cotaint::taint_block_return;

// Using a while loop is supported, this is expected to print out "[0, 1, 2, 3, 4]"
fn main() {
    let mut my_vec = taint_block_return!({
        let vec = std::vec::Vec::new();
        create_taint(vec)
    });

    taint_block!({
        let mut counter = 0;

        while counter < 5 {
            std::vec::Vec::push(extract_taint_mut_ref(&mut my_vec), counter);
            counter = counter + 1;
        }
    });

    println!("{:?}", my_vec.sanitize(|taint| taint));
}
