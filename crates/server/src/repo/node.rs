use chrono::Utc;
use models::r#do::node::{Node, NodeReversion};
use sqlx::{Acquire, AnyConnection};
use util::DataObject;

/// 创建新节点
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

/// 删除节点
pub async fn delete(conn:&mut AnyConnection,node:Node)->result::Result<()>{
    let mut tx=conn.begin().await?;
    NodeReversion::delete_by_nid(&mut *tx,&[node.nid]).await?;
    node.delete(&mut *tx).await?;
    tx.commit().await?;
    Ok(())

}