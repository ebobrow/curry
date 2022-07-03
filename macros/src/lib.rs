use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat, ReturnType};

// #[derive(Debug)]
// struct PartialInput {
//     f: Ident,
//     arg: Ident,
// }

// impl Parse for PartialInput {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let thing = input.parse()?;
//     }
// }

/// ```rust
/// let f = |a, b, c| a + b + c;
/// partial! { f 1 2 } // -> |c| 1 + 2 + c
/// partial! { f 1 } // -> |b, c| 1 + b + c
/// ```
// #[proc_macro]
// pub fn partial(item: TokenStream) -> TokenStream {
//     let item = parse_macro_input!(item as PartialInput);
//     let expanded = quote! {
//         {
//             move |arg|
//         }
//     };
//     TokenStream::from(expanded)
// }

/// ```rust
/// #[curry]
/// fn add3(a: i32, b: i32, c: i32) -> i32 { a + b + c }
/// ```
/// expands to
/// ```rust
/// fn add3(a: i32) -> Box<dyn Fn(i32) -> Box<dyn Fn(i32) -> i32>> {
///     Box::new(move |b| Box::new(move |c| a + b + c))
/// }
/// ```
#[proc_macro_attribute]
pub fn curry(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as ItemFn);

    let args = &item.sig.inputs;
    let orig_body = &item.block;
    let body = args
        .iter()
        .skip(1)
        .rev()
        .fold(quote! { #orig_body }, |acc, arg| {
            let ident = match arg {
                FnArg::Typed(pat_ty) => {
                    if let Pat::Ident(ident) = &*pat_ty.pat {
                        ident
                    } else {
                        panic!("seems wrong")
                    }
                }
                FnArg::Receiver(_) => unimplemented!("TODO"),
            };
            quote! {
                Box::new(move |#ident| #acc)
            }
        });

    let ident = &item.sig.ident;
    let input = &item.sig.inputs[0];
    let orig_output = match &item.sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => quote! { #ty },
    };
    // TODO: what if it is `pub` or whatnot
    let output = args.iter().skip(1).rev().fold(orig_output, |acc, arg| {
        let ty = match arg {
            FnArg::Typed(pat_ty) => &*pat_ty.ty,
            FnArg::Receiver(_) => unimplemented!("TODO"),
        };
        quote! {
            Box<dyn Fn(#ty) -> #acc>
        }
    });

    TokenStream::from(quote! {
        fn #ident(#input) -> #output {
            #body
        }
    })
}
