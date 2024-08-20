use chrono::{DateTime, Utc};
use sea_query::{enum_def};
use serde::Deserialize;
use uuid::Uuid;
use util::{DataObject, FromAnyRow};



#[enum_def]
#[derive(FromAnyRow,Deserialize,DataObject)]
pub struct Role{
    rid:Uuid,
    name:String,
    desc:String,
    is_base:bool,
    created:DateTime<Utc>,
    updated:DateTime<Utc>,
    changed:DateTime<Utc>
}
