use crate::r#do::utils::AppEntity;
use chrono::{DateTime, Utc};
use sea_query::enum_def;
use util::{DataObject, FromAnyRow};
use uuid::Uuid;
use sea_query_binder::SqlxBinder;
#[enum_def]
#[derive(FromAnyRow,serde::Deserialize,Clone,DataObject)]
pub struct User{
    pub uid: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub active_rid: Option<Uuid>,
    pub created:DateTime<Utc>,
    pub changed:DateTime<Utc>,
}

impl AppEntity for User {
    const RESOURCE: &'static str = "USER";
}