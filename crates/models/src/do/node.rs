use sea_query::enum_def;
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use util::{DataObject, FromAnyRow};
use uuid::Uuid;

#[enum_def]
#[derive(Deserialize,DataObject,FromAnyRow)]
pub struct NodeType{
    pub ntid:Uuid,
    pub name:String,
    pub schema:String
}