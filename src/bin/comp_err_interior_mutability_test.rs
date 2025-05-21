#![feature(auto_traits, negative_impls)]

// Will not compile - interior mutability
fn main() {
    let mut my_interior_mutable_var: String = "hello ".to_string();
    let my_interior_mutable_taint = taint_macros::source!(&mut my_interior_mutable_var);
    my_interior_mutable_var.push_str("world");
    println!("{}", my_interior_mutable_taint.sanitize(|taint| taint));
}
