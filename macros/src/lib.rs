use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt, __private::TokenStream as TokenStream2};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Colon, Comma, Dot2, FatArrow},
    Error, Expr, ExprRange, Ident, RangeLimits,
};

#[derive(Debug)]
struct NoBody;

impl Parse for NoBody {
    fn parse(_: ParseStream) -> syn::Result<Self> {
        Ok(Self)
    }
}

#[derive(Debug)]
struct Block<B> {
    body: B,
}

impl Parse for Block<Fields> {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        let input;
        braced!(input in _input);
        Ok(Self {
            body: input.parse()?,
        })
    }
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

        let addr: syn::Result<(Expr, Expr)> = match &self.lhs {
            Expr::Range(r) => match r {
                ExprRange {
                    from: Some(from),
                    to: Some(to),
                    limits,
                    ..
                } => {
                    let off = *from.clone();
                    let width = match limits {
                        RangeLimits::HalfOpen(_) => parse_quote!(#to - #from),
                        RangeLimits::Closed(_) => parse_quote!((#to - #from + 1)),
                    };
                    Ok((off, width))
                }
                ExprRange { from: None, .. } => Err(Error::new(r.span(), "range-from required")),
                ExprRange { to: None, .. } => Err(Error::new(r.span(), "range-to required")),
            },

            e => Ok((e.clone(), parse_quote!(1))),
        };

        match addr {
            Err(e) => tokens.append_all(e.into_compile_error()),
            Ok((off, width)) => tokens.append_all(quote! {
                ::svd::Field {
                    name: #name,
                    desc: #desc,
                    bit_offset: #off,
                    bit_width: #width,
                }
            }),
        };
    }
}

type Register = ArrowBlock<Fields>;

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
                fields: #fields,
            }
        })
    }
}

struct Fields {
    fields: Punctuated<Field, Comma>,
    base: Option<Ident>,
}

impl Parse for Fields {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let base = input.peek(Dot2).then(|| {
            Dot2::parse(input).unwrap();
            let id = Ident::parse(input);
            _ = Comma::parse(input);
            id
        });

        let base = match base {
            None => None,
            Some(r) => Some(r?),
        };

        Ok(Self {
            fields: Punctuated::parse_terminated(input)?,
            base,
        })
    }
}

impl ToTokens for Fields {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let fields = &self.fields;
        let base = &self.base;
        let base = base.as_ref().map_or(quote!(None), |id| quote! {Some(&#id)});

        tokens.append_all(quote! {
            ::svd::Fields {
                fields: &[#fields],
                base: #base,
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

#[proc_macro]
pub fn fields(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as Fields);
    f.to_token_stream().into()
}
