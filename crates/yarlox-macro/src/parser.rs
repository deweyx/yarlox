use proc_macro2::{Group, Ident, TokenStream, TokenTree};
use quote::TokenStreamExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Token, Type};

pub struct FieldSpec {
    pub name: Ident,
    pub ty: Type,
    pub is_self_referential: bool,
}

impl Parse for FieldSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        input.parse::<Token![:]>()?;
        let mut type_tokens = TokenStream::new();
        while !input.is_empty() && !input.peek(Token![,]) && !input.peek(Token![;]) {
            type_tokens.append(input.parse::<TokenTree>()?);
        }
        let (substituted_tokens, is_self_referential) = replace_self_tokens(type_tokens, "__alias");
        let ty: Type = syn::parse2(substituted_tokens)?;

        Ok(FieldSpec {
            name,
            ty,
            is_self_referential,
        })
    }
}

fn replace_self_tokens(stream: TokenStream, replacement: &str) -> (TokenStream, bool) {
    let mut replaced = false;

    let new_stream = stream
        .into_iter()
        .map(|tt| match tt {
            TokenTree::Ident(ident) if ident == "Self" || ident == "self" => {
                replaced = true;
                TokenTree::Ident(Ident::new(replacement, ident.span()))
            }
            TokenTree::Group(group) => {
                let (substituted, inner_replaced) = replace_self_tokens(group.stream(), replacement);
                if inner_replaced {
                    replaced = true;
                }
                TokenTree::Group(Group::new(group.delimiter(), substituted))
            }
            other => other,
        })
        .collect();

    (new_stream, replaced)
}

pub struct StructSpec {
    pub name: Ident,
    pub fields: Punctuated<FieldSpec, Token![,]>,
    pub is_self_referential: bool,
}

impl Parse for StructSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let with_token: Ident = input.parse()?;
        if with_token != "with" {
            return Err(syn::Error::new(with_token.span(), format!("expected `with`, found `{}`", with_token)));
        }

        let fields = Punctuated::<FieldSpec, Token![,]>::parse_separated_nonempty_with(input, FieldSpec::parse )?;
        let is_self_referential = fields.iter().any(|f| f.is_self_referential);

        Ok(StructSpec {
            name,
            fields,
            is_self_referential,
        })
    }
}

pub struct MacroArgs {
    pub structs: Punctuated<StructSpec, Token![;]>,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let structs = Punctuated::<StructSpec, Token![;]>::parse_terminated(input)?;
        Ok(MacroArgs { structs })
    }
}
