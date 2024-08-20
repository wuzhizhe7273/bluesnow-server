use chrono::Utc;
use sea_query::{Expr, Query};
use sqlx::{Acquire, AnyConnection};
use util::DataObject;
use uuid::Uuid;
use models::{r#do::user::{User}};
use models::r#do::api::{Api, ApiIden};
use models::r#do::role::{Role, RoleIden};
use models::r#do::role_mtm_api::RoleMtmAPiIden;
use models::r#do::user::UserIden;
use models::r#do::user_mtm_role::UserMtmRoleIden;

use sea_query_binder::SqlxBinder;
use util::query::get_query_builder;

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
pub async  fn get_related_roles(conn:&mut AnyConnection,uid:Uuid)->result::Result<Vec<Role>>{
    let (query,values)=
        Query::select()
        // select all fields
        .columns([
            RoleIden::Rid,
            RoleIden::Name,
            RoleIden::Desc,
            RoleIden::IsBase,
            RoleIden::Created,
            RoleIden::Changed
        ])
         // from user
        .from(UserIden::Table)
         // left join user.uid=user_mtm_role.uid
        .left_join(
            UserMtmRoleIden::Table,
            Expr::col((UserIden::Table,UserIden::Uid)).equals((UserMtmRoleIden::Table,UserMtmRoleIden::Uid))
        )
         // left join user_mtm_role.rid=role.rid
        .left_join(
            RoleIden::Table,
            Expr::col((UserMtmRoleIden::Table,UserMtmRoleIden::Rid)).equals((RoleIden::Table,RoleIden::Rid))
        )
         // where user.uid=${uid}
        .and_where(Expr::col((UserIden::Table,UserIden::Uid)).eq(uid)).build_any_sqlx(&*get_query_builder(conn));
    let roles=sqlx::query_as_with::<_,Role,_>(&query,values).fetch_all(conn).await?;
    Ok(roles)
}
#[allow(dead_code)]
pub async fn get_related_apis(conn:&mut AnyConnection,uid:Uuid)->result::Result<Vec<Api>>{
    let (query,values)=
        Query::select()
        // select all fields
        .columns([ApiIden::Aid,ApiIden::Name,ApiIden::Path,ApiIden::Method,ApiIden::Code,ApiIden::Created,ApiIden::Changed])
        // from user
        .from(UserIden::Table)
        // left join user.uid=user_mtm_role.uid
        .left_join(
            UserMtmRoleIden::Table,
            Expr::col((UserIden::Table,UserIden::Uid)).equals((UserMtmRoleIden::Table,UserMtmRoleIden::Uid))
        )
        // left join user_mtm_role.rid=role.rid
        .left_join(
            RoleIden::Table,
            Expr::col((UserMtmRoleIden::Table,UserMtmRoleIden::Rid)).equals((RoleIden::Table,RoleIden::Rid))
        )
        // left join role.rid=role_mtm_api.rid
        .left_join(
            RoleMtmAPiIden::Table,
            Expr::col((RoleIden::Table,RoleIden::Rid)).equals((RoleMtmAPiIden::Table,RoleMtmAPiIden::Rid))
        )
        // left join role_mtm_api.aid=api.aid
        .left_join(
            ApiIden::Table,
            Expr::col((RoleMtmAPiIden::Table,RoleMtmAPiIden::Aid)).equals((ApiIden::Table,ApiIden::Aid))
        )
        // where user.uid=${uid}
        .and_where(Expr::col((UserIden::Table,UserIden::Uid)).eq(uid)).build_any_sqlx(&*get_query_builder(conn));
    let apis=sqlx::query_as_with::<_,Api,_>(&query,values).fetch_all(conn).await?;
    Ok(apis)
}

#[allow(dead_code)]
pub async fn get_by_uid(conn: &mut AnyConnection, uid: Uuid) -> anyhow::Result<Option<User>> {
    let (query,values) = Query::select()
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
        .and_where(Expr::col(UserIden::Uid).eq(uid)).build_any_sqlx(&*get_query_builder(conn));
    let user = sqlx::query_as_with::<_, User,_>(&query,values)
        .fetch_optional(conn)
        .await?;
    Ok(user)
}

#[allow(dead_code)]
pub async fn get_by_username(
    conn: &mut AnyConnection,
    username: &str,
) -> result::Result<Option<User>> {
    let (query,values) = Query::select()
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
        .and_where(Expr::col(UserIden::Username).eq(username))
        .build_any_sqlx(&*get_query_builder(conn));


    let user = sqlx::query_as_with::<_, User,_>(&query,values)
        .fetch_optional(conn)
        .await?;
    Ok(user)
}

#[allow(dead_code)]
pub async fn delete_by_uid(conn: &mut AnyConnection, uid: Uuid) -> anyhow::Result<Uuid> {
    let conn = &mut *conn.acquire().await?;
    let query_builder=get_query_builder(conn);
    let (query,values) = Query::delete()
        .and_where(Expr::col(UserIden::Uid).eq(uid))
        .from_table(UserIden::Table).build_any_sqlx(&*query_builder);
    sqlx::query_with(&query,values).execute(conn).await?;
    Ok(uid)
}
