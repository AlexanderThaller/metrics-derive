use std::collections::HashMap;

use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::{
    format_ident,
    quote,
    ToTokens,
};
use syn::{
    parse_macro_input,
    Meta,
    PathArguments,
    Type,
};

extern crate proc_macro;

#[proc_macro_derive(Metrics, attributes(metrics))]
pub fn my_derive(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as syn::DeriveInput);

    dbg!(&parsed);

    let name = parsed.ident;

    let namespace: String = parsed
        .attrs
        .iter()
        .find_map(|attr| {
            if attr.meta.path().is_ident("metrics") {
                dbg!(attr);

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
                                found = true
                            }
                        }
                    }
                }
            }

            None
        })
        .unwrap_or(name.to_string().to_lowercase());

    dbg!(&namespace);

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

                for token in list.tokens.clone() {
                    match token {
                        TokenTree::Ident(ref ident) => field = Some(ident.to_string()),

                        TokenTree::Literal(lit) => {
                            let value = lit.to_string();
                            let value = value.trim_matches('"');
                            if let Some(f) = field {
                                entries.insert(f, value.to_owned());
                                field = None;
                            }
                        }

                        _ => {}
                    }
                }
            }
        }

        let name = entries.get("name");
        let help = entries.get("help");

        dbg!(&field_type);

        let field_type = add_generic_stuff(field_type);

        quote! {
            let #field_name = #field_type::default();
            registry.register(#name, #help, #field_name.clone());
        }
    });

    let tokens = quote! {
        impl #name {
            fn register(registry: &mut prometheus_client::registry::Registry) -> Self {
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

        dbg!(&ident);
        dbg!(&generics);

        let ts = quote! {
            #ident::#generics
        };

        syn::parse2(ts).unwrap()
    } else {
        field_type.clone()
    }
}
