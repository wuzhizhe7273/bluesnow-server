use sea_query::enum_def;
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use util::{DataObject, FromAnyRow};
use uuid::Uuid;

#[enum_def]
#[derive(FromAnyRow,Deserialize,DataObject)]
pub struct  NodeMtmTaxonomy{
    nid:Uuid,
    tid:Uuid
}