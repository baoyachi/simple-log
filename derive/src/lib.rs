use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn log_target_derive(input: TokenStream) -> TokenStream {
    let input_ident = parse_macro_input!(input as Ident);

    let parts: Vec<_> = [
        ("trace", quote!($crate::log::Level::Trace)),
        ("debug", quote!($crate::log::Level::Debug)),
        ("info", quote!($crate::log::Level::Info)),
        ("warn", quote!($crate::log::Level::Warn)),
        ("error", quote!($crate::log::Level::Error)),
    ]
    .iter()
    .map(|(level_str, level_expr)| {
        let log_target = Ident::new(
            &format!("{}_{}", level_str, input_ident),
            input_ident.span(),
        );

        quote! {
            #[macro_export]
            macro_rules! #log_target {
                ($($arg:tt)*) => {
                    $crate::log::log!(target: stringify!(#input_ident), #level_expr, $($arg)*);
                };
            }
        }
    })
    .collect();

    let combined = quote! {
        #(#parts)*
    };

    combined.into()
}
