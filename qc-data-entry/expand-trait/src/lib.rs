use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::Nothing, parse_macro_input, Ident};

struct LogLevel {
    pub ident: Ident,
    pub ident_dbg: Ident,
    pub level: String,
}

impl From<&str> for LogLevel {
    fn from(value: &str) -> Self {
        Self {
            ident: Ident::new(value, Span::call_site()),
            ident_dbg: Ident::new(&format!("{}_dbg", value), Span::call_site()),
            level: value.to_string(),
        }
    }
}

struct TraitDef {
    pub ident: Ident,
    pub level: String,
}

impl ToTokens for TraitDef {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let doc_string = format!(
            " Logs as `Display` at the `{}` level if Result is an Error.",
            &self.level
        );

        let out_tokens = quote! {
            #[doc=#doc_string]
            #[track_caller]
            fn #ident(self) -> Self;
        };

        out_tokens.to_tokens(tokens);
    }
}

impl From<&LogLevel> for TraitDef {
    fn from(value: &LogLevel) -> Self {
        Self {
            ident: value.ident.clone(),
            level: value.level.clone(),
        }
    }
}

struct TraitDefDebug {
    pub ident: Ident,
    pub level: String,
}

impl ToTokens for TraitDefDebug {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let doc_string = format!(
            " Logs as `Debug` at the `{}` level if Result is an Error.",
            &self.level
        );

        let out_tokens = quote! {
            #[doc=#doc_string]
            #[track_caller]
            fn #ident(self) -> Self;
        };

        out_tokens.to_tokens(tokens);
    }
}

impl From<&LogLevel> for TraitDefDebug {
    fn from(value: &LogLevel) -> Self {
        Self {
            ident: value.ident_dbg.clone(),
            level: value.level.clone(),
        }
    }
}

struct TraitImpl {
    pub ident: Ident,
}

impl ToTokens for TraitImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;

        let out_tokens = quote! {
        #[inline(never)]
        #[track_caller]
        fn #ident(self) -> Self {
            let caller = core::panic::Location::caller();
            self.inspect_err(|err|
            log::#ident!("Err at {caller}:\n  `{err}`"))
        }
        };

        out_tokens.to_tokens(tokens);
    }
}

impl From<&LogLevel> for TraitImpl {
    fn from(value: &LogLevel) -> Self {
        Self {
            ident: value.ident.clone(),
        }
    }
}

struct TraitImplDebug {
    pub ident: Ident,
    pub level: Ident,
}

impl ToTokens for TraitImplDebug {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let log = &self.level;

        let out_tokens = quote! {
            #[inline(never)]
            #[track_caller]
            fn #ident(self) -> Self {
                // self.inspect_err(result_error)
                let caller = core::panic::Location::caller();
                self.inspect_err(|err|
                log::#log!("Err at {caller}:\n  `{err:?}`"))
            }
        };

        out_tokens.to_tokens(tokens);
    }
}

impl From<&LogLevel> for TraitImplDebug {
    fn from(value: &LogLevel) -> Self {
        Self {
            ident: value.ident_dbg.clone(),
            level: value.ident.clone(),
        }
    }
}

#[proc_macro]
pub fn log_results(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _ = parse_macro_input!(input as Nothing);

    let log_levels: Vec<LogLevel> = vec!["error", "warn", "info", "debug", "trace"]
        .into_iter()
        .map(|x| x.into())
        .collect();
    let trait_defs: Vec<TraitDef> = log_levels.iter().map(|x| x.into()).collect();
    let trait_defs_dbg: Vec<TraitDefDebug> = log_levels.iter().map(|x| x.into()).collect();
    let trait_impls: Vec<TraitImpl> = log_levels.iter().map(|x| x.into()).collect();
    let trait_impls_dbg: Vec<TraitImplDebug> = log_levels.iter().map(|x| x.into()).collect();

    quote! {
        pub trait ResultLog {
            #(#trait_defs)*
        }
        pub trait ResultLogDebug {
            #(#trait_defs_dbg)*
        }

        impl<T, E: core::fmt::Display> ResultLog for Result<T, E> {
            #(#trait_impls)*
        }
        impl<T, E: core::fmt::Debug> ResultLogDebug for Result<T, E> {
            #(#trait_impls_dbg)*
        }
    }
    .into()
}
