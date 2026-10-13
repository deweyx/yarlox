use proc_macro2::Span;
use std::collections::BTreeSet;
use syn::visit::Visit;
use syn::{Lifetime, Type};

#[derive(Default)]
struct LifetimeVisitor {
    pub lifetimes: BTreeSet<String>,
}

impl<'a> Visit<'a> for LifetimeVisitor {
    fn visit_lifetime(&mut self, lt: &'a Lifetime) {
        let name = lt.ident.to_string();
        if name != "static" && name != "_" {
            self.lifetimes.insert(format!("'{name}"));
        }
    }
}

pub fn extract_lifetimes(types: &Vec<&Type>) -> Vec<Lifetime> {
    let mut visitor = LifetimeVisitor::default();
    for ty in types {
        visitor.visit_type(ty);
    }
    let lifetime_tokens = visitor.lifetimes.iter().map(|name| {
        Lifetime::new(&format!("{}", name), Span::call_site())
    });
    lifetime_tokens.collect()
}