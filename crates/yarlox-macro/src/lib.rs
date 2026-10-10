mod parser;
mod lifetime_visitor;

use crate::parser::MacroArgs;
use quote::quote;
use syn::ItemTrait;
use syn::parse_macro_input;
use crate::lifetime_visitor::extract_lifetimes;

#[proc_macro_attribute]
pub fn generate_ast(attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as ItemTrait);
    let mut struct_idents: MacroArgs = parse_macro_input!(attr);
    let trait_name = &input.ident;
    let impls = struct_idents.structs.iter_mut().map(|s| {
        let ty = &s.name;
        let field_names = s.fields.iter().map(|f| &f.name);
        let field_types: Vec<_> = s.fields.iter().map(|f| &f.ty).collect();

        let mut alias = quote!();
        let mut where_clause = quote!();
        let mut generics = quote!();
        if s.is_self_referential {
            alias = quote!(__alias);
            where_clause = quote!(where #alias : #trait_name)
        };
        let lifetimes = extract_lifetimes(&field_types);
        if s.is_self_referential | !lifetimes.is_empty() { generics = quote!(<#lifetimes #alias>); };

        quote! {
            struct #ty #generics #where_clause {
                #(#field_names: #field_types),*
            }
            impl #generics #trait_name for #ty #generics #where_clause { }
        }
    });

    let expanded = quote! {
        #input
        #(#impls)*
    };

    proc_macro::TokenStream::from(expanded)
}
