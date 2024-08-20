use chrono::Utc;
use models::r#do::meta_info::MetaInfo;
use models::r#do::role::Role;
use models::r#do::utils::ToResult;
use models::{r#do::user::User, req::user::RegisterRequest};
use util::pwd;
use util::DataObject;
use uuid::Uuid;

use crate::context::ServerContext;


/// 注册超级用户的接口
pub async fn init_super_user(context: ServerContext, req: RegisterRequest) -> result::Result<()> {
    let conn = &mut context.db.begin().await?;
    // 只有在未初始化时可以访问
    if MetaInfo::get(conn).await?.initialized{
        return Err(result::Error::PermissionDenied)
    }
    let uid = Uuid::now_v7();
    let now = Utc::now();
    let password = pwd::hash(&req.password).await?;
    let super_user = Role::get_by_name(conn, "SuperUser").await?.to_result()?;
    User {
        uid,
        email: req.email,
        username: req.username,
        password,
        active_rid: None,
        created: now,
        changed: now,
    }
    .save(conn)
    .await?
    .add_roles(conn, &vec![super_user])
    .await?;
    Ok(())
}
