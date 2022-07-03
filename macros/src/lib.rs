use proc_macro::TokenStream;
use quote::{__private::Span, quote};
use syn::{
    parse::Parse, parse_macro_input, Expr, ExprPath, FnArg, Ident, ItemFn, Pat, ReturnType, Token,
};

enum Arg {
    Arg(Expr),
    Elided,
}

struct PartialInput {
    f: ExprPath,
    args: Vec<Arg>,
}

impl Parse for PartialInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let f = input.parse()?;
        let mut args = Vec::new();
        loop {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![_]) {
                input.parse::<Expr>()?;
                args.push(Arg::Elided);
            } else if let Ok(arg) = input.parse() {
                args.push(Arg::Arg(arg));
            } else {
                break;
            }
        }
        Ok(Self { f, args })
    }
}

/// ```rust
/// let f = |a, b, c| a + b + c;
/// partial! { f 1 2 _ } // -> |c| f(1, 2, c)
/// partial! { f 1 _ _ } // -> |b, c| f(1, b, c)
/// ```
#[proc_macro]
pub fn partial(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as PartialInput);
    let f = &item.f;
    // let args = &item.args;
    let closure_args = item
        .args
        .iter()
        .enumerate()
        .filter(|(_, arg)| if let Arg::Elided = arg { true } else { false })
        .map(|(i, _)| Ident::new(&format!("x{}", i)[..], Span::call_site()));
    let call_args = item.args.iter().enumerate().map(|(i, arg)| match arg {
        Arg::Elided => {
            let ident = Ident::new(&format!("x{}", i)[..], Span::call_site());
            quote! { #ident }
        }
        Arg::Arg(expr) => {
            quote! { #expr }
        }
    });
    TokenStream::from(quote! {
        |#(#closure_args),*| #f(#(#call_args),*)
    })
}

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
