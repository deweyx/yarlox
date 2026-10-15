mod parser;
mod lifetime_visitor;

use crate::parser::MacroArgs;
use quote::{quote, ToTokens};
use syn::{parse_quote, ItemEnum, Lifetime};
use syn::parse_macro_input;
use crate::lifetime_visitor::extract_lifetimes;

#[proc_macro_attribute]
pub fn generate_ast(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(item as ItemEnum);
    let struct_spec: MacroArgs = parse_macro_input!(attr);

    struct_spec.structs.iter().for_each(|s| {
        let ty = &s.name;
        let field_names: Vec<_> = s.fields.iter().map(|f| &f.name).collect();
        let field_types: Vec<_> = s.fields.iter().map(|f| &f.ty).collect();

        let lifetimes = extract_lifetimes(&field_types);

        input.variants.push(parse_quote! {
            #ty { #(#field_names: #field_types),* }
        });
        lifetimes.into_iter().for_each(|l| {
            let lifetime: Lifetime = parse_quote!(#l);
            if !input.generics.lifetimes().any(|check| check.lifetime == lifetime) {
                input.generics.params.insert(0, parse_quote!(#l))
            }
        });

    });

    proc_macro::TokenStream::from(quote!{#input})
}