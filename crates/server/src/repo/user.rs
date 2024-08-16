use crate::repo::util::{NowDatabase, ToSqlxQuery};
use sea_query::{ConditionalStatement, Expr, Query};
use sqlx::{Acquire, Any, AnyConnection};
use uuid::Uuid;
use models::r#do::user::UserDO;

pub async fn register<'a, A>(conn: A, username: &str, pwd: &str) -> anyhow::Result<Uuid>
where
    A: Acquire<'a, Database = NowDatabase>,
{
    let conn = &mut *conn.acquire().await?;
    let uid = Uuid::now_v7();
    let query = Query::insert()
        .into_table(iden::User::Table)
        .columns([iden::User::Uid, iden::User::Username, iden::User::Password])
        .values([uid.into(), username.into(), pwd.into()])?
        .to_sqlx_query(conn)
        .await?;
    sqlx::query(&query).execute(conn).await?;
    Ok(uid)
}

pub async fn get_by_uid<'a, A>(conn: A, uid: Uuid) -> anyhow::Result<Option<UserDO>>
where
    A: Acquire<'a, Database = NowDatabase>,
{
    let conn = &mut *conn.acquire().await?;
    let query = Query::select()
        .columns([
            iden::User::Uid,
            iden::User::Username,
            iden::User::Email,
            iden::User::Password,
            iden::User::ActiveRid,
            iden::User::Created,
            iden::User::Created,
        ])
        .from(iden::User::Table)
        .and_where(Expr::col(iden::User::Uid).eq(uid))
        .to_sqlx_query(conn)
        .await?;
    let user=sqlx::query_as::<_,UserDO>(&query).fetch_optional(conn).await?;
    Ok(user)
}

pub async fn delete_by_uid<'a,A>(conn:A,uid:Uuid)->anyhow::Result<Uuid>
where
    A:Acquire<'a,Database=NowDatabase>
{
    let conn=&mut *conn.acquire().await?;
    let query=Query::delete()
        .and_where(Expr::col(iden::User::Uid).eq(uid))
        .from_table(iden::User::Table)
        .to_sqlx_query(conn).await?;
    sqlx::query(&query).execute(conn).await?;
    Ok(uid)
}