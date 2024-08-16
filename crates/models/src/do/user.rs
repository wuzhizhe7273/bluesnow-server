use chrono::{DateTime, FixedOffset};
use uuid::Uuid;
#[derive(sqlx::FromRow)]
pub struct UserDO{
    uid: Uuid,
    username: String,
    password: String,
    email: String,
    active_rid: Option<Uuid>,
    created: DateTime<FixedOffset>,
    changed: DateTime<FixedOffset>
}