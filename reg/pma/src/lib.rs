extern crate registry;

extern crate proc_macro;
extern crate syn;
extern crate quote;
extern crate rand;
extern crate serenity;

use proc_macro::TokenStream;
use quote::ToTokens;
use rand::RngExt;
use serenity::all::{CommandOptionType, CreateCommand, CreateCommandOption};
use syn::{FnArg, ItemFn, Pat, parse_macro_input};
use phf::phf_map;

const ARG_TYPE: phf::Map<&'static str, CommandOptionType> = phf_map! {
    // String Types
    "&str" => CommandOptionType::String,
    "String" => CommandOptionType::String,
    /* Numeric Types */
    // Signed Ints
    "i8" => CommandOptionType::Integer,
    "i16" => CommandOptionType::Integer,
    "i32" => CommandOptionType::Integer,
    "i64" => CommandOptionType::Integer,
    "i128" => CommandOptionType::Integer,
    // Unsigned Ints
    "u8" => CommandOptionType::Integer,
    "u16" => CommandOptionType::Integer,
    "u32" => CommandOptionType::Integer,
    "u64" => CommandOptionType::Integer,
    "u128" => CommandOptionType::Integer,
    // Floats
    "f16" => CommandOptionType::Number,
    "f32" => CommandOptionType::Number,
    "f64" => CommandOptionType::Number,
    "f128" => CommandOptionType::Number,
    // Boolean
    "bool" => CommandOptionType::Boolean,
    // Discord Types
    "User" => CommandOptionType::User,
    "Member" => CommandOptionType::User,
    "Channel" => CommandOptionType::Channel,
    "Role" => CommandOptionType::Role,
    "Mentionable" => CommandOptionType::Mentionable,
    "Attatchment" => CommandOptionType::Attachment
};

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
    let mut slash_cmd: CreateCommand = CreateCommand::new(&key);
    slash_cmd = slash_cmd.description("test");
    for arg in &input_fn.sig.inputs {
        if let FnArg::Typed(pat_type) = arg {
            if is_slash_command {
                println!("registering slash command!");
                let t_s: String = pat_type.ty.to_token_stream().to_string();
                
                let t_v: Vec<&str> = t_s.split(" ").collect();
                let mut t: &str = t_v[t_v.len() - 1];
                let mut is_req: bool = true;
                println!("{t}");
                println!("{}", &key);
                if t.starts_with("Option<") {
                    t = &t[7..t.len()-1];
                    is_req = false;
                }
                if let Some(cmd_opt_type) = ARG_TYPE.get(t) {
                    slash_cmd = slash_cmd.add_option(CreateCommandOption::new(*cmd_opt_type, (*pat_type.clone().pat).to_token_stream().to_string(), "...").required(is_req));
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

    if is_slash_command {
        registry::register_acmd_create(slash_cmd);
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
                ::reg::register_acmd(&#key[1..], #shim_name)
            } else if #key.starts_with("!") {
                ::reg::register_cmd(&#key[1..], #shim_name)
            } else {
                ::reg::register_event(#key, #shim_name);
            }
        }
    };

    TokenStream::from(expanded)
}