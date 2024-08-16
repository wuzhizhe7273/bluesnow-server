use sea_query::{PostgresQueryBuilder, Query};
use sqlx::{prelude::FromRow, Acquire, Any};

#[derive(FromRow)]
pub struct Node{
    nid:String
}
// pub async fn get<'a,A>(conn:A)
// where 
// A:Acquire<'a,Database = Any>
// {
//     let  conn=&mut *conn.acquire().await.unwrap();
//     let query=Query::select()
//         .column(iden::Node::Nid)
//         .from(iden::Node::Table)
//         .to_string(PostgresQueryBuilder);
//     let a=sqlx::query_as::<_,Node>(&query).fetch_one(conn).await.unwrap();
// }

// pub async fn test(pool:sqlx::AnyPool){
//     let tx=&mut pool.begin().await.unwrap();
//     get( tx).await;
// }