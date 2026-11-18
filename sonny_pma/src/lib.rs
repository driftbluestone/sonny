use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input, FnArg, Pat, Type};

/// Converts a function into a form that can be passed into an AppCommandBuilder.
#[proc_macro_attribute]
pub fn app_command(_: TokenStream, input: TokenStream) -> TokenStream {
    let func: ItemFn = parse_macro_input!(input as ItemFn);
    let vis = &func.vis;
    let fn_name = &func.sig.ident;

    let block = &func.block;

    let mut unpack_stmts = Vec::new();

    for (index, arg) in func.sig.inputs.iter().enumerate() {
        match arg {
            FnArg::Typed(pat_type) => {
                let pat: &Pat = &pat_type.pat;
                let ty: &Type = &pat_type.ty;

                // Generate: let arg_name: ArgType = *args_iter.next()...downcast::<ArgType>()...;
                unpack_stmts.push(quote! {
                    let #pat: #ty = *__args_iter
                        .next()
                        .unwrap_or_else(|| panic!("Missing argument at index {}", #index))
                        .downcast::<#ty>()
                        .unwrap_or_else(|_| panic!(
                            "Type mismatch for argument at index {}: expected `{}`",
                            #index,
                            stringify!(#ty)
                        ));
                });
            }
            FnArg::Receiver(_) => {
                return syn::Error::new_spanned(
                    arg,
                    "dynamic_handler does not support `self` parameters",
                )
                .to_compile_error()
                .into();
            }
        }
    }

    let expanded = quote! {
    #vis fn #fn_name(
        args: Vec<Box<dyn std::any::Any + Send + Sync>>
    ) -> ::std::pin::Pin<::std::boxed::Box<dyn ::std::future::Future<Output = ()> + Send + Sync + 'static>> {
        ::std::boxed::Box::pin(async move {
            let mut __args_iter = args.into_iter();
            #(#unpack_stmts)*
            #block;
            ()
        })
    }
};
    TokenStream::from(expanded)
}