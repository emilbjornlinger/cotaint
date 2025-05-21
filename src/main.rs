#![feature(auto_traits, negative_impls)]
use std::fs;

mod taint;

fn main() {
    let my_taint = taint_macros::source!("user input".to_string());
    let my_num_taint = taint_macros::source!(3);
    let my_file_taint = taint_macros::source!(fs::read_to_string("input.txt").unwrap());

    println!("{}", my_taint.sanitize(|a| a));
    println!(
        "5 + sanitized input: {}",
        5 + my_num_taint.sanitize(|a| a + 5)
    );
    println!(
        "file_taint input: {}",
        my_file_taint.sanitize(|a| format!("{a} sanitized"))
    );

    // let mut a = String::new();
    // a.push_str("innocent");


    // ====== EXAMPLES =======

    // Sanitize variable
    let my_var = taint_block_return!({ create_taint(str::to_string("innocent")) });
    let my_sanitized = my_var.sanitize(|taint| {
        let ret = taint + " sanitized";
        ret
    });
    println!("{my_sanitized}");

    // Won't compile - side effect
    // let mut my_var = taint_block_return!({ create_taint(str::to_string("innocent")) });
    // let mut my_string = "other".to_string();
    // taint_block!({
    //     std::string::String::push_str(extract_taint_mut_ref(&mut my_var), " taint");
    //     std::string::String::push_str(&mut my_string, " taint");
    // });

    // Won't compile - use of method on taints
    let mut my_var = taint_block_return!({ create_taint(str::to_string("innocent")) });
    taint_block!({
        std::string::String::push_str(extract_taint_mut_ref(&mut my_var), " taint");
        extract_taint_mut_ref(&mut my_var).push_str(" taint");
    });

    // Won't compile - use of method on safe variable // TODO
    // let mut my_var = taint_block_return!({ create_taint(str::to_string("innocent")) });
    // taint_block!({
    //     std::string::String::push_str(extract_taint_mut_ref(&mut my_var), " taint");
    //     extract_taint_mut_ref(&mut my_var).push_str(" taint");
    // });

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
    let my_sanitized = my_var.sanitize(|taint| {
        let ret = taint + " sanitized";
        ret
    });
    println!("{my_sanitized}");

    // For-loop supported
    let mut my_vec_for = taint_block_return!({
        let vec = std::vec::Vec::new();
        create_taint(vec)
    });
    taint_block!({
        let mut vec = std::vec::Vec::new();
        std::vec::Vec::push(&mut vec, 1);
        std::vec::Vec::push(&mut vec, 2);
        std::vec::Vec::push(&mut vec, 3);

        for x in vec {
            std::vec::Vec::push(extract_taint_mut_ref(&mut my_vec_for), x);
        }
    });
    println!("{:?}", my_vec_for.sanitize(|taint| taint));

    // While-loop and AssignOp supported
    let mut my_vec_while = taint_block_return!({
        let vec = std::vec::Vec::new();
        create_taint(vec)
    });
    taint_block!({
        let mut counter = 0;

        while counter < 5 {
            std::vec::Vec::push(extract_taint_mut_ref(&mut my_vec_while), counter);
            counter += 1;
        }
    });
    println!("{:?}", my_vec_while.sanitize(|taint| taint));
}
