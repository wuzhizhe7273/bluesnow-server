use heck::ToUpperCamelCase;
use quote::{format_ident, quote, ToTokens};
use syn::{
    punctuated::Punctuated,
    token::Comma,
    Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident,
};
pub struct DataObjectContext {
    ident: Ident,
    fields: Punctuated<Field, Comma>,
}

impl DataObjectContext {
    pub fn new(input: DeriveInput) -> Self {
        let ident = input.ident;
        let fields = if let Data::Struct(DataStruct {
            fields: Fields::Named(FieldsNamed { named, .. }),
            ..
        }) = input.data
        {
            named
        } else {
            panic!("Unsupported data type")
        };
        Self { ident, fields }
    }
    pub fn get_columns(self) -> Vec<Ident> {
        self.fields.clone()
            .into_iter()
            .map(|filed| {
                let field_name=filed.ident.into_token_stream().to_string().to_upper_camel_case();
                format_ident!("{}",field_name)

            })
            .collect::<Vec<_>>()
    }
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let enum_ident=format_ident!("{}Iden",ident.to_string().to_upper_camel_case());
        let field_name=self.fields.iter().map(|field|field.ident.clone().unwrap()).collect::<Vec<_>>();
        let enum_variants=field_name.iter().map(|name|format_ident!("{}",name.into_token_stream().to_string().to_upper_camel_case())).collect::<Vec<_>>();
        quote! {
            impl DataObject for #ident{
                async fn save(self, conn: &mut sqlx::AnyConnection) -> result::Result<()> {
                   let query=util::sea_query_statement_to_string!(
                       sea_query::Query::insert()
                       .into_table(#enum_ident::Table)
                       .columns([#(#enum_ident::#enum_variants),*])
                       .values([#(Into::into(self.#field_name)),*])?;
                       conn
                   );
                    sqlx::query(&query).execute(conn).await?;
                    Ok(())
                }
            }
        }
    }
}
