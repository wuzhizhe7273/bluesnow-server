use chrono::{DateTime, Utc};
use sea_query::enum_def;
use serde::Deserialize;
use uuid::Uuid;
use util::{DataObject, FromAnyRow};
use sea_query_binder::SqlxBinder;

#[enum_def]
#[derive(FromAnyRow,Deserialize,DataObject)]
pub struct Api {
    aid: Uuid,
    name: String,
    path: String,
    method: String,
    code: String,
    created: DateTime<Utc>,
    changed: DateTime<Utc>,
}
