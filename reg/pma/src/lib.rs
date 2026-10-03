extern crate registry;

extern crate proc_macro;
extern crate syn;
extern crate quote;
extern crate rand;
extern crate serenity;

use proc_macro::TokenStream;
use quote::ToTokens;
use rand::RngExt;
use syn::{FnArg, ItemFn, Pat, parse_macro_input};

#[proc_macro_attribute]
pub fn cmd(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut temp = attr.to_string().trim().to_string();
    temp = "!".to_string() + &temp;
    event(temp.parse().unwrap(), item)
}

#[proc_macro_attribute]
pub fn acmd(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut temp = attr.to_string().trim().to_string();
    temp = "/".to_string() + &temp;
    event(temp.parse().unwrap(), item)
}

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

    let is_slash_command: bool = key.starts_with("/");
    let mut args: Vec<String> = Vec::new();
    let mut is_reqs: Vec<bool> = Vec::new();
    let mut pat_types: Vec<String> = Vec::new();
    for arg in &input_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            if is_slash_command {
                let t: String = pat_type.ty.to_token_stream().to_string();
                
                let t: std::str::Split<'_, &str> = t.split(" ");
                let t: Vec<&str> = t.collect();
                let mut t: &str = t[t.len() - 1];
                let mut is_req: bool = true;
                if t.starts_with("Option<") {
                    t = &t[7..t.len()-1];
                    is_req = false;
                }
                
                if let Some(_) = registry::ARG_TYPE.get(t) {
                    args.push(t.to_string());
                    is_reqs.push(is_req);
                    pat_types.push((*pat_type.clone().pat).to_token_stream().to_string());
                }
            }

            if let Pat::Ident(pat_ident) = &*pat_type.pat {
                let arg_ident = &pat_ident.ident;
                let arg_type = &pat_type.ty;

                downcasts.push(quote::quote! {
                    let #arg_ident = *args_iter.next()
                        .expect("Macro error: Missing argument")
                        .downcast::<#arg_type>()
                        .expect("Macro error: Argument type mismatch");
                });
                arg_names.push(quote::quote! { #arg_ident });
            }
        }
    }

    let expanded = quote::quote! {
        #fn_vis #input_fn


        fn #shim_name(args: Vec<Box<dyn std::any::Any + Send + Sync>>) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'static>> {
            Box::pin(async move {
                let mut args_iter = args.into_iter();
                #(#downcasts)*

                #fn_name(#(#arg_names),*).await;
            })
        }

        #[::reg::__private::ctor::ctor(unsafe, crate_path = ::reg::__private::ctor)]
        fn #ctor_name() {
            if #is_slash_command {
                let args_rt: Vec<(&str, bool, &str)> = vec![
                    #( (#args, #is_reqs, #pat_types) ),*
                ];
                let arg_types: Vec<String> = vec![
                    #( #args.to_string(),),*
                ];
                let mut slash_cmd: serenity::all::CreateCommand = serenity::all::CreateCommand::new(&#key[1..]);
                slash_cmd = slash_cmd.description("...");
                for (arg, is_req, pat_type) in args_rt {
                    if let Some(cmd_opt_type) = ::reg::ARG_TYPE.get(arg) {
                        slash_cmd = slash_cmd.add_option(serenity::all::CreateCommandOption::new(*cmd_opt_type, pat_type, "...").required(is_req));
                    }
                }
                ::reg::register_acmd_create(slash_cmd, &#key[1..], arg_types);
                ::reg::register_acmd(&#key[1..], #shim_name);
            } else if #key.starts_with("!") {
                ::reg::register_cmd(&#key[1..], #shim_name);
            } else {
                ::reg::register_event(#key, #shim_name);
            }
        }
    };

    TokenStream::from(expanded)
}