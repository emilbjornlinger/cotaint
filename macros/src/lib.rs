use proc_macro::TokenStream;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use std::iter::FromIterator;
use syn::{
    parse_macro_input, spanned::Spanned, token::Comma, Block, Expr, ExprField, FieldValue, Stmt,
};

// TODO Write docs
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

// TODO Write docs
#[proc_macro]
pub fn taint_block(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block);

    let generated_closure = taint_block_helper(&block);

    quote! {
        ::cotaint::taint::closure_guard<_>(
            #generated_closure
        )
    }
    .into()
}

// TODO Write docs
#[proc_macro]
pub fn taint_block_return(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block);

    let generated_closure = taint_block_helper(&block);

    quote! {
        ::cotaint::taint::closure_guard_return<_>(
            #generated_closure
        )
    }
    .into()
}

// TODO Write docs
fn taint_block_helper(input_block: &Block) -> TokenStream {
    let body: TokenStream = match &*block {
        Expr::Block(b) => expand_block(&b.block).into(),
        _ => quote! {compile_error!("Expected block in taint_block* macro")}.into(),
    };

    let generated_code = if cfg!(debug_assertions) {
        quote! {
            (|| -> _ {
                let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| { #body })).unwrap_or_default();
                result
            })
        }
    } else {
        quote! {
            (|| -> _ {
                let prev_hook = ::ssecret_block_backend_helpertd::panic::take_hook();
                ::std::panic::set_hook(Box::new(|_| {}));
                let result = ::std::panic::catch_unwind(::std::panic::AssertUnwindSafe(|| { #body })).unwrap_or_default();
                ::std::panic::set_hook(prev_hook);
                result
            })
        }
    };

    generated_code.into()
}

// TODO Write docs
fn expand_block(block: &syn::Block) -> TokenStream {
    let token_streams: Vec<TokenStream> = block
        .stmts
        .iter()
        .map(|stmt: &Stmt| -> TokenStream {
            match stmt {
                Stmt::Local(local_expr) => match &local_expr.init {
                    // Check the right-hand side of a store.
                    Some((_, expr)) => {
                        let mut new_expr = local_expr.clone();
                        let new_init: Expr = syn::parse(expand_expr(expr).into()).unwrap();
                        new_expr.init = Some((syn::token::Eq(expr.span()), Box::new(new_init)));
                        quote! {
                            #new_expr
                        }
                    }
                    None => local_expr.into_token_stream().into(),
                },
                Stmt::Item(item) => {
                    // Looking at the definition of Item, any Item should be fine.
                    item.into_token_stream().into()
                }
                Stmt::Expr(expr) => expand_expr(expr),
                Stmt::Semi(expr, _) => {
                    let expr_tokens = expand_expr(expr);
                    quote! {
                        #expr_tokens;
                    }
                }
            }
        })
        .collect();
    let stream: TokenStream = TokenStream::from_iter(token_streams);
    let gen = quote! {
        {
            #stream
        }
    };
    gen.into()
}

// TODO Write docs
fn expand_expr(expr: &Expr) -> TokenStream {
    match expr {
        Expr::Array(expr_array) => {
            let elements = comma_separate(expr_array.elems.iter().map(|expr| expand_expr(expr)));
            quote! {
                [#elements]
            }
            .into()
        }
        Expr::Break(expr_break) => expr_break.into_token_stream(),
        Expr::Call(expr_call) => {
            let args = comma_separate(
                expr_call
                    .args
                    .iter()
                    .map(|arg: &syn::Expr| -> TokenStream { expand_expr(arg) }),
            );
            if is_call_to(expr_call, "extract_taint_ref") {
                quote! {
                    { let tmp = #args; unsafe { ::cotaint::Tainted::extract_as_ref(tmp) } }
                }
            } else if is_call_to(expr_call, "extract_taint_mut_ref") {
                quote! {
                    { let tmp = #args; unsafe { ::cotaint::Tainted::extract_as_mut_ref(tmp) } }
                }
            } else if is_call_to(expr_call, "extract_taint") {
                quote! {
                    { let tmp = #args; unsafe { ::cotaint::Tainted::extract_and_consume(tmp) } }
                }
            } else if is_call_to(expr_call, "encapsule_taint") {
                quote! {
                    { let tmp = #args; unsafe { ::cotaint::Tainted::new(tmp) } }
                }
            } else if is_call_to_allowed_functions(expr_call) {
                let func = &*expr_call.func;
                quote! {
                    #func(#args)
                }
            } else {
                let func = &*expr_call.func;
                quote! {
                    { compile_error!("This func is unsupported {:?}", #func) } //Might not print out a pretty result, probably need to extract the function's attributes
                }
                .into()
            }
        }
        Expr::Binary(expr_binary) => {
            // Check the left-hand side of the expression, and the right-hand side.
            let lhs = expand_expr(&*expr_binary.left);
            let rhs = expand_expr(&*expr_binary.right);
            let op = expr_binary.op;
            quote! {
                #lhs #op #rhs
            }
        }
        Expr::Block(expr_block) => expand_block(&expr_block.block).into(),
        Expr::Continue(continue_stmt) => continue_stmt.into_token_stream(),
        Expr::Assign(assign_expr) => {
            let lhs: TokenStream = expand_expr(&assign_expr.left).into();
            let rhs: TokenStream = expand_expr(&assign_expr.right).into();
            // Don't need not_mut_secret here since it's already in the duplicate (i.e., check_expr) path
            quote! {
                (#lhs = #rhs)
            }
        }
        Expr::AssignOp(assign_op_expr) => {
            let lhs: TokenStream = expand_expr(&assign_op_expr.left).into();
            let rhs: TokenStream = expand_expr(&assign_op_expr.right).into();
            let op = assign_op_expr.op;
            // Don't need not_mut_secret here since the duplicate (i.e., check_expr) path ensures built-in numeric/string types
            quote! {
                (#lhs #op #rhs)
            }
        }
        Expr::MethodCall(method_call_expr) => {
            let receiver: TokenStream = expand_expr(&method_call_expr.receiver).into();
            let args = comma_separate(
                method_call_expr
                    .args
                    .iter()
                    .map(|arg: &syn::Expr| -> TokenStream { expand_expr(arg) }),
            );
            let method = &method_call_expr.method;
            let turbofish = &method_call_expr.turbofish;
            // TODO: Shouldn't evaluate #args inside of unsafe block -- ADDED WHITESPACE AFTER METHOD, caused error
            quote! {
                //(unsafe { (#receiver.#method #turbofish(#args) as ::secret_structs::secret::Vetted<_>).unwrap() })
            }
        }
        Expr::If(expr_if) => {
            let condition = expand_expr(&*expr_if.cond);
            let then_block: TokenStream = expand_block(&expr_if.then_branch).into();
            let else_branch = match &expr_if.else_branch {
                Some(block) => expand_expr(&*block.1),
                None => quote! {},
            };
            quote! {
                if #condition {
                    #then_block
                } else {
                    #else_branch
                }
            }
        }
        Expr::Field(field_access) => {
            let e: Expr = syn::parse2(expand_expr(&(field_access.base))).expect("ErrS");
            let e2: Expr = syn::parse2(quote! { (#e) }).expect("ErrS");
            let e3: Box<Expr> = Box::new(e2);
            let f_new = ExprField {
                attrs: field_access.attrs.clone(),
                base: e3,
                dot_token: field_access.dot_token.clone(),
                member: field_access.member.clone(),
            };
            f_new.into_token_stream()
        }
        Expr::ForLoop(for_loop) => {
            // TODO: Does #pat need expansion?
            let pat = for_loop.pat.clone().into_token_stream();
            let expr: TokenStream = expand_expr(&*for_loop.expr).into();
            let body: TokenStream = expand_block(&for_loop.body).into();
            quote! {
                for #pat in #expr {
                    #body
                }
            }
        }
        Expr::Index(idx) => {
            let expr: TokenStream = expand_expr(&*idx.expr).into();
            let index: TokenStream = expand_expr(&*idx.index).into();
            quote! {
                #expr[#index]
            }
        }
        Expr::Lit(expr_lit) => expr_lit.into_token_stream(),
        Expr::Match(expr_match) => {
            let mut expr_match_copy = expr_match.clone();
            expr_match_copy.expr =
                Box::new(syn::parse2(expand_expr(&*expr_match_copy.expr)).unwrap());
            for arm in &mut expr_match_copy.arms {
                match &arm.guard {
                    Some((if_token, guard_expr_boxed)) => {
                        arm.guard = Some((
                            *if_token,
                            Box::new(syn::parse2(expand_expr(&*guard_expr_boxed)).unwrap()),
                        ))
                    }
                    _ => {}
                }
                arm.body = Box::new(syn::parse2(expand_expr(&*arm.body)).unwrap());
            }
            expr_match_copy.into_token_stream()
        }
        Expr::Paren(paren_expr) => {
            let interal_expr = expand_expr(&paren_expr.expr);
            let mut new_paren_expr = paren_expr.clone();
            new_paren_expr.expr = Box::new(syn::parse2(interal_expr).unwrap());
            new_paren_expr.into_token_stream()
        }
        Expr::Path(path_access) => path_access.into_token_stream(),
        Expr::Reference(reference) => {
            let operand = expand_expr(&*reference.expr);
            match reference.mutability {
                Some(_) => {
                    quote! {
                        &mut #operand
                    }
                }
                _ => {
                    quote! {
                        &#operand
                    }
                }
            }
        }
        Expr::Return(return_expr) => {
            if let None = return_expr.expr {
                return return_expr.into_token_stream();
            }
            let mut new_return_expr = return_expr.clone();
            let expr = expand_expr(&new_return_expr.expr.unwrap());
            new_return_expr.expr = Some(Box::new(syn::parse2(expr).unwrap()));
            new_return_expr.into_token_stream()
        }
        Expr::Struct(struct_literal) => {
            let fields: syn::punctuated::Punctuated<FieldValue, Comma> = {
                let mut f = syn::punctuated::Punctuated::<FieldValue, Comma>::new();
                for field in struct_literal.clone().fields.iter() {
                    let e = syn::parse2(expand_expr(&field.expr)).expect("ErrS");
                    let fv = syn::FieldValue {
                        attrs: field.attrs.clone(),
                        member: field.member.clone(),
                        colon_token: field.colon_token.clone(),
                        expr: e,
                    };
                    f.push(fv);
                }
                f
            };
            let struct_new = syn::ExprStruct {
                attrs: struct_literal.attrs.clone(),
                path: struct_literal.path.clone(),
                brace_token: struct_literal.brace_token.clone(),
                fields: fields,
                dot2_token: struct_literal.dot2_token.clone(),
                rest: struct_literal.rest.clone(),
            };
            struct_new.into_token_stream()
        }
        Expr::Unary(unary) => {
            let operand = expand_expr(&*unary.expr);
            let operator = unary.op;
            quote! {
                #operator #operand
            }
        }
        Expr::Unsafe(unsafe_expr) => {
            quote! {#unsafe_expr}
        }
        Expr::Cast(cast) => {
            let expr = expand_expr(&*cast.expr);
            let ty = &cast.ty;
            quote! {
                #expr as #ty
            }
        }
        Expr::Range(range) => {
            let mut range_copy = range.clone();
            match range_copy.from {
                Some(from) => range_copy.from = Some(syn::parse2(expand_expr(&*from)).unwrap()),
                _ => {}
            };
            match range_copy.to {
                Some(to) => range_copy.to = Some(syn::parse2(expand_expr(&*to)).unwrap()),
                _ => {}
            };
            quote! {(#range_copy)}
        }
        Expr::Repeat(repeat_expr) => {
            let expr = expand_expr(&repeat_expr.expr);
            let len_expr = expand_expr(&repeat_expr.len);
            let mut new_repeat_expr = repeat_expr.clone();
            new_repeat_expr.expr = Box::new(syn::parse2(expr).unwrap());
            new_repeat_expr.len = Box::new(syn::parse2(len_expr).unwrap());
            new_repeat_expr.into_token_stream()
        }
        Expr::Tuple(tuple) => {
            let args = comma_separate(
                tuple
                    .elems
                    .iter()
                    .map(|arg: &syn::Expr| -> TokenStream { expand_expr(arg) }),
            );
            quote! {
                (#args)
            }
        }
        Expr::While(while_loop) => {
            let cond: TokenStream = expand_expr(&*while_loop.cond).into();
            let body: TokenStream = expand_block(&while_loop.body).into();
            quote! {
                while #cond {
                    #body
                }
            }
        }
        _expr => {
            quote! {
                { compile_error!("This expr is unsupported {:?}", _expr) } //Might not print out a pretty result, probably need to extract the expression's attributes
            }
            .into()
        } /*
          Only implemented the same expressions as Cocoon,
          these are the expressions which are not yet handled
          and will cause the compile error above.
          */
          // Expr::Async(expr_async) => todo!(),
          // Expr::Await(expr_await) => todo!(),
          // Expr::Break(expr_break) => todo!(),
          // Expr::Const(expr_const) => todo!(),
          // Expr::Closure(expr_closure) => todo!(),
          // Expr::Group(expr_group) => todo!(),
          // Expr::Loop(expr_loop) => todo!(),
          //Expr::Let(expr_let) => todo!(),
          // Expr::Macro(expr_macro) => todo!(),
          //Expr::Yield(expr_yield) => todo!(),
          //Expr::RawAddr(expr_raw_addr) => todo!(),
          //Expr::Verbatim(token_stream) => todo!(),
          //Expr::Try(expr_try) => todo!(),
          //Expr::Infer(expr_infer) => todo!(),
          //Expr::TryBlock(expr_try_block) => todo!(),
    }
}

//TODO Write docs
// Code from Cocooon
fn is_call_to_allowed_functions(call: &syn::ExprCall) -> bool {
    let allowed_functions = HashSet::from([
        "std::option::Option::unwrap".to_string(),
        "std::slice::Iter::copied".to_string(),
        "std::string::String::clear".to_string(),
        "std::string::String::from".to_string(),
        "std::string::String::len".to_string(),
        "std::string::String::clone".to_string(),
        "std::vec::Vec::clear".to_string(),
        "std::vec::Vec::clone".to_string(),
        "std::vec::Vec::extend_from_slice".to_string(),
        "std::vec::Vec::len".to_string(),
        "std::vec::Vec::new".to_string(),
        "std::vec::Vec::push".to_string(),
        "std::vec::Vec::with_capacity".to_string(),
        "std::collections::HashMap::get".to_string(),
        "std::collections::HashMap::insert".to_string(),
        "std::collections::HashMap::contains_key".to_string(),
        "std::collections::HashSet::insert".to_string(),
        "str::chars".to_string(),
        "str::to_string".to_string(),
        "str::trim".to_string(),
        "usize::to_string".to_string(),
        "<[T]>::sort".to_string(),
    ]);

    if let syn::Expr::Path(path_expr) = &*call.func {
        let mut path_str = quote! {#path_expr}.to_string();
        path_str.retain(|c| !c.is_whitespace());
        allowed_functions.contains(&path_str)
    } else {
        false
    }
}

// TODO Write docs
fn is_call_to(call: &syn::ExprCall, path: &str) -> bool {
    if let syn::Expr::Path(path_expr) = &*call.func {
        let mut path_str = quote! {#path_expr}.to_string();
        path_str.retain(|c| !c.is_whitespace());
        return path_str == path;
    } else {
        false
    }
}

// TODO Write docs
fn comma_separate<T: Iterator<Item = TokenStream>>(ts: T) -> TokenStream {
    ts.fold(
        TokenStream::new(),
        |acc: TokenStream, token: TokenStream| -> TokenStream {
            if acc.is_empty() {
                token
            } else {
                let ba: TokenStream = acc.into();
                let bt: TokenStream = token.into();
                quote! {#ba, #bt}
            }
        },
    )
}
