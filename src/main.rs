#![feature(auto_traits, negative_impls)]
use taint_macros;
use std::fs;

mod taint;

fn main() {
    unsafe {
        let my_taint = taint_macros::source!("user input".to_string());
        let my_num_taint = taint_macros::source!(3);
        let my_file_taint = taint_macros::source!(fs::read_to_string("input.txt").unwrap());

        println!("{}", my_taint.sanitize(|a| a));

        println!("5 + sanitized input: {}", 5 + my_num_taint.sanitize(|a| a + 5));

        println!("file_taint input: {}", my_file_taint.sanitize(|a| format!("{a} sanitized")));
    }

    let a = String::new();
    taint_macros::taint_block!(a.push("add"));
    taint_macros::taint_block_return!(a.push("add"));
}
