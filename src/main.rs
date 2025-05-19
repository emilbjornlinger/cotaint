#![feature(auto_traits, negative_impls)]
use std::fs;
mod taint;

fn main() {
    let my_taint = taint_macros::source!("user input".to_string());
    let my_num_taint = taint_macros::source!(3);
    let my_file_taint = taint_macros::source!(fs::read_to_string("input.txt").unwrap());

    println!("{}", my_taint.sanitize(|a| a));
    println!("5 + sanitized input: {}", 5 + my_num_taint.sanitize(|a| a + 5));
    println!("file_taint input: {}", my_file_taint.sanitize(|a| format!("{a} sanitized")));

    // let mut a = String::new();
    // a.push_str("innocent");

    let mut my_var = taint_block_return!({create_taint(str::to_string("innocent"))});
    let mut my_string = "other".to_string();
    let _my_ret = taint_block_return!({
        std::string::String::push_str(extract_taint_mut_ref(&mut my_var), " taint");
        //std::string::String::push_str(&mut my_string, " taint");
        create_taint(0)
    });

    taint_block!({
        std::string::String::push_str(extract_taint_mut_ref(&mut my_var), " taint");
    });

    let my_sanitized = my_var.sanitize(|taint| {
        let ret = taint + " sanitized";
        ret
    });

    println!("{my_sanitized}");
}
