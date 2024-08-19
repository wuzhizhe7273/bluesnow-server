use derive_data_object::DataObjectContext;
use proc_macro::TokenStream;
use crate::from_any_row::FromAnyRowContext;

mod from_any_row;
mod derive_data_object;

#[proc_macro_derive(FromAnyRow)]
pub fn from_any_row(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);
    FromAnyRowContext::new(input).generate().into()
}

#[proc_macro_derive(DataObject)]
pub fn derive_data_object(item:TokenStream)->TokenStream{
    let input=syn::parse_macro_input!(item as syn::DeriveInput);
    DataObjectContext::new(input).generate().into()
}