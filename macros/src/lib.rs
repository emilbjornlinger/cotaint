use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn taint_block(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree 
    // that we can manipulate
    let expr: syn::Expr = syn::parse(input).unwrap();

    // Build the trait implementation
    //impl_source_macro(&expr)
    quote! {
        println!("Hello from taint_block procedural macro");
    }.into()
}

#[proc_macro]
pub fn taint_block_return(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree 
    // that we can manipulate
    let expr: syn::Expr = syn::parse(input).unwrap();

    // Build the trait implementation
    //impl_source_macro(&expr)
    quote! {
        println!("Hello from taint_block_return procedural macro");
    }.into()
}

#[proc_macro]
pub fn source(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree 
    // that we can manipulate
    let expr = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_source_macro(&expr)
}

fn impl_source_macro(expr: &syn::Expr) -> TokenStream {
    let gen = quote! {
        ::cotaint::taint::Tainted::<_>::new(#expr)
    };
    gen.into()
}
