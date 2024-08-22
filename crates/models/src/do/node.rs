use super::utils::AppEntity;
use chrono::{DateTime, Utc};
use sea_query::{enum_def, Expr, Query};
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use sqlx::AnyConnection;
use util::query::get_query_builder;
use util::{DataObject, FromAnyRow};
use uuid::Uuid;

#[enum_def]
#[derive(Deserialize, DataObject, FromAnyRow)]
pub struct NodeType {
    pub ntid: Uuid,
    pub name: String,
    pub schema: String,
}

#[enum_def]
#[derive(Deserialize, DataObject, FromAnyRow, Clone)]
pub struct Node {
    pub nid: Uuid,
    pub vid: Uuid,
    pub ntid: Uuid,
    pub language: String,
    pub title: String,
    /// 置顶 NOT_STICKY| STICKY
    pub sticky: String,
    pub uid: Uuid,
    ///  节点状态 PUBLISHED |NOT_PUBLISHED
    pub status: String,
    pub changed: DateTime<Utc>,
    pub created: DateTime<Utc>,
    ///  评论状态 HIDDEN | CLOSED | OPEN
    pub comment: String,
    pub tnid: Option<Uuid>,
    pub data: Option<serde_json::Value>,
}

impl AppEntity for Node {
    const RESOURCE: &'static str = "NODE";
}

impl Node {
    pub async fn get_by_nid(conn: &mut AnyConnection, nid: Uuid) -> result::Result<Option<Self>> {
        let (query, values) = Query::select()
            .columns([
                NodeIden::Nid,
                NodeIden::Vid,
                NodeIden::Ntid,
                NodeIden::Language,
                NodeIden::Title,
                NodeIden::Sticky,
                NodeIden::Uid,
                NodeIden::Status,
                NodeIden::Changed,
                NodeIden::Created,
                NodeIden::Comment,
                NodeIden::Tnid,
                NodeIden::Data,
            ])
            .from(NodeIden::Table)
            .build_any_sqlx(*get_query_builder(conn));
        let res = sqlx::query_as_with::<_, Self, _>(&query, values)
            .fetch_optional(conn)
            .await?;
        Ok(res)
    }

    pub async fn delete(self, conn: &mut AnyConnection) -> result::Result<Self> {
        let (query, values) = Query::delete()
            .from_table(NodeIden::Table)
            .and_where(Expr::col(NodeIden::Nid).eq(self.nid))
            .build_any_sqlx(conn);
        sqlx::query_with(&query, values).execute(conn).await?;
        Ok(self)
    }
}

#[enum_def]
#[derive(Deserialize, DataObject, FromAnyRow)]
pub struct NodeReversion {
    pub vid: Uuid,
    pub nid: Uuid,
    pub uid: Uuid,
    pub title: String,
    pub data: Option<serde_json::Value>,
    pub log: String,
    pub timestamp: DateTime<Utc>,
}

impl AppEntity for NodeReversion {
    const RESOURCE: &'static str = "NODE_REVERSION";
}
impl NodeReversion {
    pub async fn get_by_vid(conn: &mut AnyConnection, vid: Uuid) -> result::Result<Option<Self>> {
        let (query, values) = Query::select()
            .columns([
                NodeReversionIden::Vid,
                NodeReversionIden::Nid,
                NodeReversionIden::Uid,
                NodeReversionIden::Title,
                NodeReversionIden::Data,
                NodeReversionIden::Log,
                NodeReversionIden::Timestamp
            ])
            .from(NodeReversionIden::Table)
            .build_any_sqlx(*get_query_builder(conn));
        let res = sqlx::query_as_with::<_, Self, _>(&query, values)
            .fetch_optional(conn)
            .await?;
        Ok(res)
    }

    pub async fn delete(self,conn: &mut AnyConnection) -> result::Result<Self> {
        let (query, values) = Query::delete()
            .from_table(NodeReversionIden::Table)
            .and_where(Expr::col(NodeReversionIden::Vid).eq(self.vid))
            .build_any_sqlx(conn);
        sqlx::query_with(&query, values).execute(conn).await?;
        Ok(self)
    }
}