use chrono::{DateTime, Utc};
use sea_query::enum_def;
use sea_query_binder::SqlxBinder;
use serde::Deserialize;
use util::{DataObject, FromAnyRow};
use uuid::Uuid;

use super::utils::AppEntity;

#[enum_def]
#[derive(Deserialize,DataObject,FromAnyRow)]
pub struct NodeType{
    pub ntid:Uuid,
    pub name:String,
    pub schema:String
}


#[enum_def]
#[derive(Deserialize,DataObject,FromAnyRow,Clone)]
pub struct Node{
    pub nid:Uuid,
    pub vid:Uuid,
    pub ntid:Uuid,
    pub language:String,
    pub title:String,
    /// 置顶 NOT_STICKY| STICKY
    pub sticky:String,
    pub uid:Uuid,
    ///  节点状态 PUBLISHED |NOT_PUBLISHED
    pub status:String,
    pub changed:DateTime<Utc>,
    pub created:DateTime<Utc>,
    ///  评论状态 HIDDEN | CLOSED | OPEN
    pub comment:String,
    pub tnid:Option<Uuid>,
    pub data:Option<serde_json::Value>
}

impl AppEntity for Node {
    const RESOURCE:&'static str = "NODE";
}

#[enum_def]
#[derive(Deserialize,DataObject,FromAnyRow)]
pub struct NodeReversion{
    pub vid:Uuid,
    pub nid:Uuid,
    pub uid:Uuid,
    pub title:String,
    pub data:Option<serde_json::Value>,
    pub log:String,
    pub timestamp:DateTime<Utc>
}

impl AppEntity for NodeReversion {
    const RESOURCE:&'static str = "NODE_REVERSION";
}
