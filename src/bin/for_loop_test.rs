#![feature(auto_traits, negative_impls)]

use cotaint::taint_block;
use cotaint::taint_block_return;

// Using a for loop is supported, this is expected to print out "[1, 2, 3]"
fn main() {
    let mut my_vec = taint_block_return!({
        let vec = std::vec::Vec::new();
        create_taint(vec)
    });

    taint_block!({
        let mut vec = std::vec::Vec::new();
        std::vec::Vec::push(&mut vec, 1);
        std::vec::Vec::push(&mut vec, 2);
        std::vec::Vec::push(&mut vec, 3);

        for x in vec {
            std::vec::Vec::push(extract_taint_mut_ref(&mut my_vec), x);
        }
    });

    println!("{:?}", my_vec.sanitize(|taint| taint));
}
