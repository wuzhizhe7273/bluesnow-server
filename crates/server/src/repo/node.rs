use chrono::Utc;
use models::r#do::node::{Node, NodeReversion};
use sqlx::{Acquire, AnyConnection};
use util::DataObject;

/// 必须传入事务
pub async fn create(conn: &mut AnyConnection, node: Node) -> result::Result<Node> {
    let mut tx =conn.begin().await?;
    NodeReversion {
        vid: node.vid,
        nid: node.nid,
        uid: node.uid,
        title: node.title.clone(),
        data: node.data.clone(),
        log: String::new(),
        timestamp: Utc::now(),
    }
    .save(&mut *tx)
    .await?;
    node.clone().save(&mut *tx).await?;
    tx.commit().await?;
    Ok(node)
}

pub async fn delete_by_nid(conn:&mut AnyConnection)->result::Result<()>{
    let mut tx=conn.begin().await?;
    todo!()
}