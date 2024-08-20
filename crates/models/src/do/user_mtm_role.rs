use sea_query::enum_def;
use serde::Deserialize;
use uuid::Uuid;
use util::{DataObject, FromAnyRow};
use sea_query_binder::SqlxBinder;

#[enum_def]
#[derive(Deserialize,DataObject,FromAnyRow)]
pub struct UserMtmRole {
    pub uid:Uuid,
    pub rid:Uuid
}
