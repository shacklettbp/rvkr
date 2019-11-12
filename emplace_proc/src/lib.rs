extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::fold::Fold;

struct ReturnVisitor {}

impl Fold for ReturnVisitor {
    fn fold_expr_return(&mut self, e: syn::ExprReturn) -> syn::ExprReturn {
        match e.expr {
            Some(ret_expr) => {
                let new_ret_expr = self.fold_expr(*ret_expr);

                syn::parse_quote! {
                    return {
                        unsafe { __out_ptr.write(#new_ret_expr); };
                    }
                }
            },
            None => panic!("new method returns nothing")
        }
    }
}

fn get_new_method(ast: &syn::ItemImpl) -> Option<&syn::ImplItemMethod> {
    for item in &ast.items {
        match item {
            syn::ImplItem::Method(m) if m.sig.ident.to_string() == "new" =>
                return Some(m),
            _ => () 
        }
    }

    None
}

fn is_return(expr: &syn::Expr) -> bool {
    match expr {
        syn::Expr::Return(..) => true,
        _ => false
    }
}

fn rewrite_implicit_return(s: syn::Stmt) -> syn::Stmt {
    match s {
        syn::Stmt::Expr(e) if !is_return(&e) => {
            syn::parse_quote! {
                unsafe { __out_ptr.write(#e) };
            }
        },
        _ => s
    }
}

fn impl_emplace(ast: syn::ItemImpl) -> TokenStream {
    let struct_ty = ast.self_ty.clone();

    let new = get_new_method(&ast).expect("InplaceNew needs new method");

    let fn_args = &new.sig.inputs;
    let call_args : Vec<_> = fn_args.iter().map(|arg| {
        if let syn::FnArg::Typed(syn::PatType { pat, .. }) = arg {
            if let syn::Pat::Ident(pat_ident) = &**pat {
                return &pat_ident.ident;
            }
        }
        panic!("Unsupported function argument in new definition")
    }).collect();

    let mut ret_folder = ReturnVisitor {};
    let mut new_block = ret_folder.fold_block(new.block.clone());
    let last_stmt = new_block.stmts.pop().expect("Empty new method");
    new_block.stmts.push(rewrite_implicit_return(last_stmt));

    let ty_str = quote! { #struct_ty }.to_string();

    let box_trait_name = quote::format_ident!("{}_BoxEmplace", ty_str);
    let vec_trait_name = quote::format_ident!("{}_VecEmplace", ty_str);

    let gen = quote! {
        #ast

        impl #struct_ty {
            #[inline(always)]
            pub fn new_emplace(__out_ptr: *mut #struct_ty, #fn_args)
            #new_block
        }

        trait #box_trait_name {
            fn emplace(#fn_args) -> Box<#struct_ty>;
        }

        impl #box_trait_name for Box<#struct_ty> {
            fn emplace(#fn_args) -> Box<#struct_ty> {
                use std::alloc::{alloc, Layout};
                let ptr = unsafe { alloc(Layout::new::<#struct_ty>()) as
                    *mut #struct_ty };

                <#struct_ty>::new_emplace(ptr, #(#call_args),*);

                unsafe { Box::from_raw(ptr) }
            }
        }

        trait #vec_trait_name {
            fn push_emplace(&mut self, #fn_args);
        }

        impl #vec_trait_name for Vec<#struct_ty> {
            fn push_emplace(&mut self, #fn_args) {
                self.reserve(1);
                let __idx = self.len();

                let __ptr = unsafe { self.as_mut_ptr().add(__idx) };
                <#struct_ty>::new_emplace(__ptr, #(#call_args),*);

                unsafe { self.set_len(__idx + 1); }
            }
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn emplace(_: TokenStream, item: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(item as syn::ItemImpl);

    impl_emplace(ast)
}

