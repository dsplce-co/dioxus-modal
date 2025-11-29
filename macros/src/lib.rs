use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn modal(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;
    let fn_name = &sig.ident;
    let fn_args = &sig.inputs;

    let new_component_name = format_ident!("_{}", fn_name);

    let arg_names = fn_args.iter().filter_map(|arg| {
        if let syn::FnArg::Typed(pat_type) = arg {
            if let syn::Pat::Ident(ident) = &*pat_type.pat {
                Some(ident.ident.clone())
            } else {
                None
            }
        } else {
            None
        }
    });

    let generated = quote! {
        #[allow(non_snake_case)]
        #vis fn #fn_name(#fn_args) -> Element {
            rsx! {
                #new_component_name {
                    #(#arg_names),*
                }
            }
        }

        #[component]
        fn #new_component_name(#fn_args) -> Element {
            #block
        }
    };

    generated.into()
}
