#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
//#![warn(clippy::unwrap_used)]
#![warn(rust_2018_idioms, unused_lifetimes, missing_debug_implementations)]

use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{
    quote,
    ToTokens,
};
use syn::{
    parse_macro_input,
    Meta,
    Type,
};

/// # Panics
/// TODO
#[allow(clippy::too_many_lines)]
#[proc_macro_derive(Metrics, attributes(metrics))]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as syn::DeriveInput);

    let name = parsed.ident;

    let namespace: String = parsed
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.meta.path().is_ident("metrics") {
                if let Meta::List(ref list) = attr.meta {
                    let mut found = false;
                    let mut skipped = false;
                    for token in list.tokens.clone() {
                        if found && !skipped {
                            skipped = true;
                            continue;
                        }

                        if found && skipped {
                            if let TokenTree::Literal(lit) = token {
                                let namespace = lit.to_string();
                                let namespace = namespace.trim_matches('"');
                                return Some(namespace.to_string());
                            }
                        }

                        if let TokenTree::Ident(ref ident) = token {
                            if ident == "namespace" {
                                found = true;
                            }
                        }
                    }
                }
            }

            None
        })
        .unwrap_or(name.to_string().to_lowercase());

    let fields = match parsed.data {
        syn::Data::Struct(data) => match data.fields {
            syn::Fields::Named(named) => named.named,
            syn::Fields::Unnamed(_) => unimplemented!(),
            syn::Fields::Unit => unimplemented!(),
        },
        syn::Data::Enum(_) => unimplemented!(),
        syn::Data::Union(_) => unimplemented!(),
    };

    let fields_struct = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { #name }
    });

    let metrics_fields = fields.iter().map(|f| {
        let field_name = &f.ident;
        let field_type = &f.ty;
        let field_attrs = &f.attrs;

        let mut entries = HashMap::new();

        for attr in field_attrs {
            if !attr.meta.path().is_ident("metrics") {
                continue;
            }

            if let Meta::List(ref list) = attr.meta {
                let mut field = None;
                let mut section = None;

                for token in list.tokens.clone() {
                    match token {
                        TokenTree::Ident(ref ident) => {
                            if field.is_none() {
                                field = Some(ident.to_string());
                            } else {
                                section = Some(ident.to_string());
                            }
                        }

                        TokenTree::Literal(lit) => {
                            let value = lit.to_string();
                            let value = value.trim_matches('"');
                            if let Some(f) = field {
                                entries.insert(f, value.to_owned());
                                field = None;
                            }
                        }

                        TokenTree::Group(group) => {
                            let mut value = section.unwrap().to_string();
                            value.push_str(&group.to_string());

                            if let Some(f) = field {
                                entries.insert(f, value.clone());
                                field = None;
                            }

                            section = None;
                        }

                        TokenTree::Punct(_) => {}
                    }
                }
            }
        }

        let name = entries
            .remove("name")
            .unwrap_or_else(|| field_name.as_ref().unwrap().to_string());
        let name = format!("{namespace}_{name}");

        let help = entries.remove("help");
        let init = entries.remove("init");
        let set = entries.remove("set");
        let buckets = entries.remove("buckets");

        let metric_type = field_type.to_token_stream().to_string();

        let field_type = add_generic_stuff(field_type);

        let init_function = if metric_type.contains("Histogram") {
            let buckets = if let Some(b) = buckets {
                format!("[{b}]")
            } else {
                "[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]".to_string()
            };

            let buckets: proc_macro2::TokenStream = buckets
                .parse()
                .expect("Failed to parse string into TokenStream");

            quote! {
                #field_type::new(#buckets.into_iter())
            }
        } else {
            quote! { #field_type::default() }
        };

        if let Some(init) = init {
            let init: proc_macro2::TokenStream = init
                .parse()
                .expect("Failed to parse string into TokenStream");

            let set: proc_macro2::TokenStream = if let Some(set) = set {
                set.parse()
                    .expect("Failed to parse string into TokenStream")
            } else {
                "1".parse()
                    .expect("Failed to parse string into TokenStream")
            };

            quote! {
                let #field_name = #init_function;
                registry.register(#name, #help, #field_name.clone());
                #field_name.get_or_create(&#init).set(#set);
            }
        } else {
            quote! {
                let #field_name = #init_function;
                registry.register(#name, #help, #field_name.clone());
            }
        }
    });

    let tokens = quote! {
        impl #name {
            pub fn register(registry: &mut prometheus_client::registry::Registry) -> Self {
                #(#metrics_fields)*

                Self {
                    #(#fields_struct),*
                }
            }
        }
    };

    TokenStream::from(tokens)
}

fn add_generic_stuff(field_type: &Type) -> Type {
    if !field_type.to_token_stream().to_string().contains('<') {
        return field_type.clone();
    }

    if let Type::Path(type_path) = field_type {
        let segment = &type_path.path.segments.last().unwrap();
        let ident = &segment.ident;
        let generics = &segment.arguments;

        let ts = quote! {
            #ident::#generics
        };

        syn::parse2(ts).unwrap()
    } else {
        field_type.clone()
    }
}
