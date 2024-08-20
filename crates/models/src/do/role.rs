use super::utils::AppEntity;
use crate::r#do::api::{Api, ApiIden};
use chrono::{DateTime, Utc};
use sea_query::{enum_def, Expr, Query};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use sqlx::AnyConnection;
use util::{query::get_query_builder, DataObject, FromAnyRow};
use uuid::Uuid;

#[enum_def]
#[derive(FromAnyRow, Deserialize, DataObject)]
pub struct Role {
    pub rid: Uuid,
    pub name: String,
    pub desc: String,
    pub is_base: bool,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
}

impl AppEntity for Role {
    const RESOURCE: &'static str = "ROLE";
}

impl Role {
    pub async fn get_by_name(conn: &mut AnyConnection, name: &str) -> result::Result<Option<Self>> {
        let (query, values) = Query::select()
            // select all columns
            .columns([
                RoleIden::Rid,
                RoleIden::Name,
                RoleIden::Desc,
                RoleIden::IsBase,
                RoleIden::Created,
                RoleIden::Changed,
            ])
            .from(RoleIden::Table)
            .and_where(Expr::col(RoleIden::Name).eq(name))
            .build_any_sqlx(&*get_query_builder(conn));
        let role = sqlx::query_as_with::<_, Self, _>(&query, values)
            .fetch_optional(conn)
            .await?;
        Ok(role)
    }

    pub async fn get_related_api(self, conn: &mut AnyConnection) -> result::Result<Vec<Api>> {
        let (query, values) = Query::select()
            // select all columns
            .columns([
                ApiIden::Aid,
                ApiIden::Name,
                ApiIden::Path,
                ApiIden::Method,
                ApiIden::Code,
                ApiIden::Created,
                ApiIden::Changed,
            ])
            .from(RoleIden::Table)
            .left_join(
                ApiIden::Table,
                Expr::col((RoleIden::Table, RoleIden::Rid)).equals((ApiIden::Table, ApiIden::Aid)),
            )
            .and_where(Expr::col(RoleIden::Rid).eq(self.rid))
            .build_any_sqlx(&*get_query_builder(conn));
        let apis = sqlx::query_as_with::<_, Api, _>(&query, values)
            .fetch_all(conn)
            .await?;
        Ok(apis)
    }
}
