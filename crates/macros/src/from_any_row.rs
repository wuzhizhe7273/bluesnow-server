use std::iter::Map;
use quote::quote;
use syn::{
    punctuated::{Iter, Punctuated},
    token::Comma,
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident,
};
pub struct FromAnyRowContext {
    name: Ident,
    fields: Punctuated<Field, Comma>,
}
impl FromAnyRowContext {
    pub fn new(input: DeriveInput) -> Self {
        let name = input.ident;
        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type")
        };
        Self { name, fields }
    }
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let name=&self.name;
        let parsers = self.get_parsed_fields();
        let idents = self.get_idents();
        quote! {
            impl<'r> sqlx::FromRow<'r,sqlx::any::AnyRow> for #name {
                fn from_row(row: &'r sqlx::any::AnyRow) -> Result<Self, sqlx::Error> {
                    #( #parsers ;)*
                    Ok(Self{
                        #( #idents ),*
                    })
                }
            }
        }
    }
    pub fn get_idents<'a>(
        &'a self,
    ) -> Map<Iter<'a, Field>, fn(&'a Field) -> proc_macro2::TokenStream> {
        self.fields.iter().map(|field| {
            let ident = &field.ident;
            quote! {
                #ident
            }
        })
    }
    pub fn get_parsed_fields<'a>(
        &'a self,
    ) -> Map<Iter<'a, Field>, fn(&'a Field) -> proc_macro2::TokenStream> {
        self.fields.iter().map(|field| {
            let ty = &field.ty;
            let ident = &field.ident;
            quote! {
                let #ident:#ty=row.any_parse(stringify!(#ident))?
            }
        })
    }
}
