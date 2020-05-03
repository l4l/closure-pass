extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Ident, Result, Token};

enum Arg {
    Item(Ident),
    ExprItem(Ident, Expr),
}

impl Parse for Arg {
    fn parse(input: ParseStream) -> Result<Self> {
        let ident: Ident = input.parse()?;

        let la = input.lookahead1();

        if la.peek(Token![=]) {
            let _: Token![=] = input.parse()?;
            Ok(Arg::ExprItem(ident, input.parse()?))
        } else {
            Ok(Arg::Item(ident))
        }
    }
}

struct Args(Vec<Arg>);

impl Args {
    fn clone_items(&self) -> impl Iterator<Item = &Ident> + '_ {
        self.0.iter().filter_map(|arg| {
            if let Arg::Item(ident) = arg {
                Some(ident)
            } else {
                None
            }
        })
    }

    fn arbitrary_exprs(&self) -> impl Iterator<Item = (&Ident, &Expr)> + '_ {
        self.0.iter().filter_map(|arg| {
            if let Arg::ExprItem(ident, expr) = arg {
                Some((ident, expr))
            } else {
                None
            }
        })
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(Self(Vec::new()));
        }

        let terms = input.parse_terminated::<_, Token![,]>(Arg::parse)?;

        Ok(Self(terms.into_iter().collect()))
    }
}

#[proc_macro_attribute]
pub fn closure_pass(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);
    let input = parse_macro_input!(input as Expr);

    let clone_items = args
        .clone_items()
        .map(|ident| {
            quote! {
                let #ident = #ident.clone();
            }
        })
        .collect::<Vec<_>>();

    let arbitrary_exprs = args
        .arbitrary_exprs()
        .map(|(ident, expr)| {
            quote! {
                let #ident = #expr;
            }
        })
        .collect::<Vec<_>>();

    let processed = quote! {{
        #(#clone_items)*
        #(#arbitrary_exprs)*
        #input
    }};

    TokenStream::from(processed)
}
