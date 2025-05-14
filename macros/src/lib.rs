use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr};

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

#[proc_macro]
pub fn taint_block(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as syn::Expr);

    let code = taint_block_helper(&input);

    code
}

fn taint_block_helper(expr: &syn::Expr) -> TokenStream {

    match expr {
        Expr::Block(expr_block) => {
            expand_block(expr_block)
        }
        _ => expand_expr(expr)
    }

}

fn expand_block(expr_block: &syn::ExprBlock) -> TokenStream {




    TokenStream::new()
}

fn expand_expr(expr: &Expr) -> TokenStream {
    TokenStream::new()
}