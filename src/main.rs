use taint_macros;

mod taint;

fn main() {
    let my_taint = taint_macros::source!("user input".to_string());
    let my_num_taint = taint_macros::source!(3);

    println!("{}", my_taint.sanitize());

    println!("5 + sanitized input: {}", 5 + my_num_taint.sanitize());
}
