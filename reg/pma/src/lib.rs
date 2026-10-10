extern crate registry;

extern crate proc_macro;
extern crate syn;
extern crate quote;
extern crate rand;
extern crate serenity;

use proc_macro::TokenStream;
use quote::ToTokens;
use rand::RngExt;
use syn::{Attribute, FnArg, ItemFn, Meta, Pat, parse_macro_input};

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

// These dont do anything by themselves, they only exist to prompt acmd.
#[proc_macro_attribute] pub fn acmd_desc(_attr: TokenStream, item: TokenStream) -> TokenStream {item}
#[proc_macro_attribute] pub fn arg_name(_attr: TokenStream, item: TokenStream) -> TokenStream {item}
#[proc_macro_attribute] pub fn arg_desc(_attr: TokenStream, item: TokenStream) -> TokenStream {item}

#[proc_macro_attribute]
pub fn event(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut rng = rand::rng();
    let mut key = attr.to_string().trim().to_string();
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    if key == "" || key == "!" || key == "/" {
        key += &input_fn.sig.ident.to_string();
    }

    let fn_vis = &input_fn.vis;
    let num: u128 = rng.random();
    let shim_name = quote::format_ident!("{fn_name}_s{num}");
    let ctor_name = quote::format_ident!("{fn_name}_c{num}");

    let mut downcasts = Vec::new();
    let mut arg_names = Vec::new();

    let is_slash_command: bool = key.starts_with("/");
    let mut acmd_desc: String = String::from("...");
    let mut field_names: Vec<String> = Vec::new();
    let mut field_descs: Vec<String> = Vec::new();
    if is_slash_command {
        let attrs: Vec<&Attribute> = input_fn.attrs.iter().collect();
        for attr in attrs {
            if attr.path().is_ident("acmd_desc") {
                if let Meta::List(meta_list) = &attr.meta {
                    let toks = &meta_list.tokens;
                    acmd_desc = toks.to_string();
                }
            } else if attr.path().is_ident("arg_name") {
                if let Meta::List(meta_list) = &attr.meta {
                    let toks = &meta_list.tokens;
                    field_names.push(toks.to_string());
                }
            } else if attr.path().is_ident("arg_desc") {
                if let Meta::List(meta_list) = &attr.meta {
                    let toks = &meta_list.tokens;
                    field_descs.push(toks.to_string());
                }
            }
        }
    }
    // this is poorly named but im too scared to touch it
    let mut args: Vec<String> = Vec::new(); // the type of arg
    let mut arg_descs: Vec<String> = Vec::new(); // description of the arg
    let mut is_reqs: Vec<bool> = Vec::new(); // if it is required
    let mut pat_types: Vec<String> = Vec::new(); // name of the arg in discord
    for i in 0..input_fn.sig.inputs.len() {
        let arg = &input_fn.sig.inputs[i];
        if let FnArg::Typed(pat_type) = arg {
            if is_slash_command {
                let t: String = pat_type.ty.to_token_stream().to_string();
                let mut is_req: bool = true;
                if t.starts_with("Option") {
                    is_req = false;
                }
                let t: std::str::Split<'_, &str> = t.split(" ");
                let t: Vec<&str> = t.collect();
                let t_s: &str;
                if t.len() > 1 {
                    t_s = t[t.len() - 2];
                } else {
                    t_s = t[0];
                }
                if let Some(_) = registry::ARG_TYPE.get(t_s) {
                    args.push(t_s.to_string());

                    if i < field_names.len()+2 {
                        pat_types.push(field_names[i-2].clone());
                    } else {
                        pat_types.push((*pat_type.clone().pat).to_token_stream().to_string());
                    }

                    if i < field_descs.len()+2 {
                        arg_descs.push(field_descs[i-2].clone());
                    } else {
                        arg_descs.push("...".to_string());
                    }
                    
                    is_reqs.push(is_req);
                    
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
                let args_rt: Vec<(&str, &str, bool, &str)> = vec![
                    #( (#args, #arg_descs, #is_reqs, #pat_types) ),*
                ];
                let mut arg_types: Vec<(String, String, bool)> = Vec::new();
                let mut slash_cmd: serenity::all::CreateCommand = serenity::all::CreateCommand::new(&#key[1..]);
                slash_cmd = slash_cmd.description(#acmd_desc);
                for (arg, arg_desc, is_req, pat_type) in args_rt {
                    if let Some(cmd_opt_type) = ::reg::ARG_TYPE.get(arg) {
                        slash_cmd = slash_cmd.add_option(serenity::all::CreateCommandOption::new(*cmd_opt_type, pat_type, arg_desc).required(is_req));
                        arg_types.push((pat_type.to_string(), arg.to_string(), is_req));
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