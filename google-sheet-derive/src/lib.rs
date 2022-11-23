#![warn(clippy::pedantic)]
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{self, parse_macro_input, spanned::Spanned, FieldsNamed, Ident};

#[proc_macro_derive(GoogleSheet)]
pub fn add(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input);
    impl_hello_macro(&ast)
}

#[doc(hidden)]
fn impl_named(name: &Ident, fields: &FieldsNamed) -> TokenStream {
    let recurse = fields.named.iter().map(|f| {
        let name = &f.ident;
        quote_spanned! {f.span()=>
            #name: ::google_sheet::get_data(&this_row, &indexes_for_fields, stringify!(#name))
        }
    });
    let gen = quote! {
        impl GoogleSheet<#name> for #name {
            fn from_grid_data(data: &::google_sheets4::api::GridData) -> std::result::Result<Vec<#name>, ()> {
                if let Some(row_data) = &data.row_data {
                    if let Some(first_row) = row_data.get(0) {
                        let indexes_for_fields = ::google_sheet::create_index_map(&first_row);
                        let mut result = Vec::with_capacity(row_data.len());
                        for row in 1..row_data.len() {
                            if let Some(this_row) = row_data.get(row) {
                                let item = #name {
                                    #( #recurse ),*
                                };
                                result.push(item);
                            }
                        }
                        Ok(result)
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
            }
        }
    };
    gen.into()
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    match &ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(named_fields) => impl_named(name, named_fields),
            syn::Fields::Unnamed(_) => panic!("Expected a named struct, found unnamed struct"),
            syn::Fields::Unit => panic!("Expected a named struct, found empty struct"),
        },
        syn::Data::Enum(_) => panic!("Expected a struct, found an enum"),
        syn::Data::Union(_) => panic!("Expected a struct, found a union"),
    }
}
