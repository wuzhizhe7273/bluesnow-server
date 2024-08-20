use sea_query::enum_def;
use serde::Deserialize;
use uuid::Uuid;
use util::{DataObject, FromAnyRow};

#[enum_def]
#[derive(Deserialize,DataObject,FromAnyRow)]
pub struct UserMtmRole {
    uid:Uuid,
    rid:Uuid
}
