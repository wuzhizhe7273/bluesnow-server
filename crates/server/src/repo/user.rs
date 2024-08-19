use chrono::Utc;
use sea_query::{Expr, Query};
use sqlx::{Acquire, AnyConnection};
use util::{sea_query_statement_to_string, DataObject};
use uuid::Uuid;
use models::{r#do::user::{User}};
use models::r#do::user::UserIden;

pub async fn register(
    conn: &mut AnyConnection,
    username: &str,
    email: &str,
    pwd: &str,
) -> result::Result<Uuid> {
    let uid = Uuid::now_v7();
    let now=Utc::now();
    User{
        uid,
        username:username.into(),
        password:pwd.into(),
        email:email.into(),
        active_rid:None,
        created:now,
        changed:now,
    }.save(conn).await?;
    Ok(uid)
}

#[allow(dead_code)]
pub async fn get_by_uid(conn: &mut AnyConnection, uid: Uuid) -> anyhow::Result<Option<User>> {
    let query = sea_query_statement_to_string!(Query::select()
        .columns([
            UserIden::Uid,
            UserIden::Username,
            UserIden::Email,
            UserIden::Password,
            UserIden::ActiveRid,
            UserIden::Created,
            UserIden::Created,
        ])
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Uid).eq(uid));
        conn
    );
    let user = sqlx::query_as::<_, User>(&query)
        .fetch_optional(conn)
        .await?;
    Ok(user)
}

#[allow(dead_code)]
pub async fn get_by_username(
    conn: &mut AnyConnection,
    username: &str,
) -> result::Result<Option<User>> {
    let query = sea_query_statement_to_string!(Query::select()
        .columns([
            UserIden::Uid,
            UserIden::Username,
            UserIden::Email,
            UserIden::Password,
            UserIden::ActiveRid,
            UserIden::Created,
            UserIden::Created,
        ])
        .from(UserIden::Table)
        .and_where(Expr::col(UserIden::Username).eq(username));
        conn);

    let user = sqlx::query_as::<_, User>(&query)
        .fetch_optional(conn)
        .await?;
    Ok(user)
}

#[allow(dead_code)]
pub async fn delete_by_uid(conn: &mut AnyConnection, uid: Uuid) -> anyhow::Result<Uuid> {
    let conn = &mut *conn.acquire().await?;
    let query = sea_query_statement_to_string!(Query::delete()
        .and_where(Expr::col(UserIden::Uid).eq(uid))
        .from_table(UserIden::Table);
        conn
    );
    sqlx::query(&query).execute(conn).await?;
    Ok(uid)
}
