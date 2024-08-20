use sea_query::{enum_def, Expr, Query};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use sqlx::AnyConnection;
use util::{query::get_query_builder, DataObject, FromAnyRow};
#[enum_def]
#[derive(FromAnyRow, Deserialize, DataObject)]
pub struct MetaInfo {
    pub id: String,
    pub version: String,
    pub database: String,
    pub initialized: bool,
}

impl MetaInfo {
    // 是否初始化
    pub async fn if_init(conn: &mut AnyConnection) -> result::Result<bool> {
        let (query, values) = Query::select()
            .columns([
                MetaInfoIden::Id,
                MetaInfoIden::Version,
                MetaInfoIden::Database,
                MetaInfoIden::Initialized,
            ])
            .from(MetaInfoIden::Table)
            .build_any_sqlx(&*get_query_builder(conn));
        let info: Self = sqlx::query_as_with(&query, values).fetch_one(conn).await?;
        Ok(info.initialized)
    }
    // 设置初始化状态
    pub async fn set_initialize_status(
        conn: &mut AnyConnection,
        status: bool,
    ) -> result::Result<Self> {
        let (query, values) = Query::update()
            .table(MetaInfoIden::Table)
            .value(MetaInfoIden::Initialized, status)
            .and_where(Expr::col(MetaInfoIden::Id).eq("meta"))
            .build_any_sqlx(&*get_query_builder(conn));
        let info: Self = sqlx::query_as_with(&query, values).fetch_one(conn).await?;
        Ok(info)
    }
    // 获取信息
    pub async fn get(conn: &mut AnyConnection) -> result::Result<Self> {
        let (query, values) = Query::select()
            .columns([
                MetaInfoIden::Id,
                MetaInfoIden::Version,
                MetaInfoIden::Database,
                MetaInfoIden::Initialized,
            ])
            .from(MetaInfoIden::Table)
            .build_any_sqlx(&*get_query_builder(conn));
        let info: Self = sqlx::query_as_with(&query, values).fetch_one(conn).await?;
        Ok(info)
    }
}
