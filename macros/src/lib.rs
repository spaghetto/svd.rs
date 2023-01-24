use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt, __private::TokenStream as TokenStream2};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::{self, Colon, Comma, FatArrow},
    Expr, Ident,
};

#[derive(Debug)]
struct NoBody;

impl Parse for NoBody {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self)
    }
}

#[derive(Debug)]
struct Block<B> {
    body: B,
}

impl<T: Parse, P: Parse> Parse for Block<Punctuated<T, P>> {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        let input;
        braced!(input in _input);

        let p = Punctuated::parse_terminated(&input)?;
        Ok(Self { body: p })
    }
}

impl<B: ToTokens> ToTokens for Block<B> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.body.to_tokens(tokens)
    }
}

/// Expr of form:
///     <expr> => <ident>: <expr> {}
#[derive(Debug)]
struct ArrowMap<B> {
    lhs: Expr,
    ident: Ident,
    rhs: Expr,
    body: B,
}

impl<B: Parse> Parse for ArrowMap<B> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lhs: Expr = input.parse()?;
        FatArrow::parse(input)?;
        let ident: Ident = input.parse()?;
        Colon::parse(input)?;
        let rhs: Expr = input.parse()?;
        let body: B = input.parse()?;

        Ok(Self {
            lhs,
            ident,
            rhs,
            body,
        })
    }
}

type ArrowBlock<B> = ArrowMap<Block<B>>;

type Field = ArrowMap<NoBody>;

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.ident.to_string();
        let desc = &self.rhs;
        let bit_offset = &self.lhs;

        tokens.append_all(quote! {
            ::svd::Field {
                name: #name,
                desc: #desc,
                bit_offset: #bit_offset,
                bit_width: 1,
            }
        })
    }
}

type Register = ArrowBlock<Punctuated<Field, Comma>>;

impl ToTokens for Register {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let name = self.ident.to_string();
        let desc = &self.rhs;
        let addr = &self.lhs;
        let fields = &self.body;

        tokens.append_all(quote! {
            ::svd::Register {
                name: #name,
                desc: #desc,
                addr: #addr,
                fields: &[#fields],
            }
        })
    }
}

type Peripheral = ArrowBlock<Punctuated<Register, Comma>>;

impl ToTokens for Peripheral {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Peripheral {
            lhs,
            rhs,
            ident,
            body,
        } = self;

        let name = ident.to_string();

        tokens.append_all(quote! {
            ::svd::Peripheral {
                name: #name,
                desc: #rhs,
                addr: #lhs,
                regs: &[#body],
            }
        })
    }
}

#[proc_macro]
pub fn peripheral(input: TokenStream) -> TokenStream {
    let p = parse_macro_input!(input as Peripheral);
    p.to_token_stream().into()
}
