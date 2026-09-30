extern crate proc_macro;
extern crate syn;
extern crate quote;
extern crate registry;
extern crate ctor;
extern crate rand;
extern crate serenity;
use proc_macro::TokenStream;
use rand::RngExt;
use syn::{parse_macro_input, ItemFn, FnArg, Pat};

#[proc_macro_attribute]
pub fn event(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut rng = rand::rng();
    let key = attr.to_string().trim().to_string();
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_vis = &input_fn.vis;

    let num: u128 = rng.random();
    let shim_name = quote::format_ident!("{fn_name}_s{num}");
    let ctor_name = quote::format_ident!("{fn_name}_c{num}");

    let mut downcasts = Vec::new();
    let mut arg_names = Vec::new();

    for (idx, arg) in input_fn.sig.inputs.iter().enumerate() {
    if let FnArg::Typed(pat_type) = arg {
        if let Pat::Ident(pat_ident) = &*pat_type.pat {
            let arg_ident = &pat_ident.ident;
            let arg_type = &pat_type.ty;

            if let syn::Type::Reference(type_ref) = &**arg_type {
                if type_ref.mutability.is_some() {
                    let inner_type = &type_ref.elem;
                    
                    downcasts.push(quote::quote! {
                        let arc_wrapper = args[#idx]
                            .downcast_ref::<std::sync::Arc<serenity::prelude::RwLock<#inner_type>>>()
                            .expect("Macro error: Argument type mismatch");
                        
                        let mut #arg_ident = arc_wrapper.write().await;
                    });
                    arg_names.push(quote::quote! { &mut *#arg_ident });
                    continue;
                }
            }
            
            arg_names.push(quote::quote! { #arg_ident });
        }
    }
}

    let expanded = quote::quote! {
        // Keep the user's clean, original function intact
        #fn_vis #input_fn

        // Generate the complex shim function that the registry actually expects
        fn #shim_name(mut args: Vec<Box<dyn std::any::Any + Send + Sync>>) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
            Box::pin(async move {
                // 1. Unbox and downcast all variables
                #(#downcasts)*

                // 2. Await the user's original async function
                #fn_name(#(#arg_names),*).await;
            })
        }

        // Register the SHIM instead of the original function
        #[ctor::ctor(unsafe)]
        fn #ctor_name() {
            registry::register(#key, #shim_name);
        }
    };

    TokenStream::from(expanded)
}