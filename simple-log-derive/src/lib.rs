use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn log_target_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Ident);

    const LOG_LEVELS: [&str; 5] = ["trace", "debug", "info", "warn", "error"];

    let mut parts = Vec::new();

    for level in LOG_LEVELS.iter() {
        let log_target = Ident::new(&format!("{}_{}", level, input), input.span());
        let log_level = match *level {
            "trace" => quote!($crate::log::Level::Trace),
            "debug" => quote!($crate::log::Level::Debug),
            "info" => quote!($crate::log::Level::Info),
            "warn" => quote!($crate::log::Level::Warn),
            "error" => quote!($crate::log::Level::Error),
            _ => unreachable!(),
        };

        let expand = quote! {
            #[macro_export]
            macro_rules! #log_target {
                ($($arg:tt)*) => ($crate::log::log!(target: stringify!(#input), #log_level, $($arg),*));
            }
        };
        parts.push(expand);
    }

    let combined = quote! {
        #(#parts)*
    };

    TokenStream::from(combined)
}
