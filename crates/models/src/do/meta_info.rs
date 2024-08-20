use sea_query::enum_def;
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use util::{DataObject, FromAnyRow};
#[enum_def]
#[derive(FromAnyRow,Deserialize,DataObject)]
pub struct MetaInfo{
    pub version: String,
    pub database: String,
    pub initialized: bool,
}