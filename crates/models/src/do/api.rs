use chrono::{DateTime, Utc};
use sea_query::enum_def;
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use util::{DataObject, FromAnyRow};
use uuid::Uuid;

#[enum_def]
#[derive(FromAnyRow,Deserialize,DataObject)]
pub struct  Api {
    pub aid: Uuid,
    pub name: String,
    pub path: String,
    pub method: String,
    pub code: String,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
}
