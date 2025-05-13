use taint_macros;
use std::fs;

mod taint;

fn main() {
    let my_taint = taint_macros::source!("user input".to_string());
    let my_num_taint = taint_macros::source!(3);
    let my_file_taint = taint_macros::source!(fs::read_to_string("input.txt"));

    println!("{}", my_taint.sanitize());

    println!("5 + sanitized input: {}", 5 + my_num_taint.sanitize());

    println!("file_taint input: {}", my_file_taint.sanitize().unwrap());
}
