use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Expr, ExprLit, Lit};

// Uses syn (v2) to parse the macro #[aoc_main(1)]
#[proc_macro_attribute]
pub fn aoc_main(_args: TokenStream, item: TokenStream) -> TokenStream {
    let solve_fn = parse_macro_input!(item as syn::ItemFn);

    let solve_fn_identifier = solve_fn.sig.ident.clone();

    quote! {
        #solve_fn

        fn main() {
            let input_file = std::env::args().nth(1).expect("Expect input file as first argument");
            let solution: Solution = #solve_fn_identifier(Input::new(&input_file)).into();
            println!("{}", solution);
            solution.copy_to_clipboard();
        }
    }
    .into()
}

#[proc_macro_derive(CollectFromStr, attributes(sep))]
pub fn collect_from_str(item: TokenStream) -> TokenStream {
    let s = parse_macro_input!(item as syn::DeriveInput);
    let ident = s.ident;

    let sep = s
        .attrs
        .iter()
        .find_map(|attr| {
            let nv = attr.meta.require_name_value().unwrap();
            if nv.path.is_ident("sep") {
                match &nv.value {
                    Expr::Lit(ExprLit { lit, .. }) => match lit {
                        Lit::Str(lit_str) => Some(lit_str.value()),
                        Lit::Char(lit_char) => Some(lit_char.value().to_string()),
                        _ => None,
                    },
                    _ => None,
                }
            } else {
                None
            }
        })
        .unwrap_or(",".to_string());

    quote! {
        impl std::str::FromStr for #ident {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(
                    Self(
                        s.trim()
                          .split(#sep)
                          .enumerate()
                          .map(|(i, e)| e.trim().parse().expect(&format!("Failed to parse element {i} ({e}) of {s}")))
                          .collect()
                    )
                )
            }
        }
    }
    .into()
}

#[proc_macro_derive(HashMapFromStr, attributes(sep, inner_sep, reverse))]
pub fn hash_map_from_str(item: TokenStream) -> TokenStream {
    let s = parse_macro_input!(item as syn::DeriveInput);
    let ident = s.ident;

    let sep = s
        .attrs
        .iter()
        .find_map(|attr| {
            let nv = attr.meta.require_name_value().unwrap();
            if nv.path.is_ident("sep") {
                match &nv.value {
                    Expr::Lit(ExprLit { lit, .. }) => match lit {
                        Lit::Str(lit_str) => Some(lit_str.value()),
                        Lit::Char(lit_char) => Some(lit_char.value().to_string()),
                        _ => None,
                    },
                    _ => None,
                }
            } else {
                None
            }
        })
        .unwrap_or(",".to_string());

    let inner_sep = s
        .attrs
        .iter()
        .find_map(|attr| {
            let nv = attr.meta.require_name_value().unwrap();
            if nv.path.is_ident("inner_sep") {
                match &nv.value {
                    Expr::Lit(ExprLit { lit, .. }) => match lit {
                        Lit::Str(lit_str) => Some(lit_str.value()),
                        Lit::Char(lit_char) => Some(lit_char.value().to_string()),
                        _ => None,
                    },
                    _ => None,
                }
            } else {
                None
            }
        })
        .unwrap_or(":".to_string());

    let reverse = s
        .attrs
        .iter()
        .any(|attr| attr.meta.path().is_ident("reverse"));
    let reverse = if reverse {
        quote! { true }
    } else {
        quote! { false }
    };

    quote! {
        impl std::str::FromStr for #ident {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use itertools::Itertools;
                Ok(
                    Self(
                        s.trim()
                          .split(#sep)
                          .enumerate()
                          .map(|(i, e)| {
                              let (mut key, mut val) = e.trim().split(#inner_sep).collect_tuple().unwrap();
                              if (#reverse) {
                                  (key, val) = (val, key);
                              }
                              (
                                  key.trim().parse().expect(&format!("Failed to parse key ({key}) for element {i} ({e}) of {s}")),
                                  val.trim().parse().expect(&format!("Failed to parse value ({val}) for element {i} ({e}) of {s}"))
                              )
                          })
                          .collect()
                    )
                )
            }
        }
    }
    .into()
}
