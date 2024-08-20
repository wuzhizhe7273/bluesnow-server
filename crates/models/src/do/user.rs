use crate::r#do::utils::AppEntity;
use chrono::{DateTime, Utc};
use sea_query::{enum_def, Expr, Query};
use sea_query_binder::SqlxBinder;
use sqlx::AnyConnection;
use util::{query::get_query_builder, DataObject, FromAnyRow};
use uuid::Uuid;

use super::{
    api::{Api, ApiIden},
    role::{Role, RoleIden},
    role_mtm_api::RoleMtmAPiIden,
    user_mtm_role::{UserMtmRole, UserMtmRoleIden},
};
#[enum_def]
#[derive(FromAnyRow, serde::Deserialize, Clone, DataObject)]
pub struct User {
    pub uid: Uuid,
    pub username: String,
    pub password: String,
    pub email: String,
    pub active_rid: Option<Uuid>,
    pub created: DateTime<Utc>,
    pub changed: DateTime<Utc>,
}

impl AppEntity for User {
    const RESOURCE: &'static str = "USER";
}

impl User {
    // 添加角色
    pub async fn add_roles(
        self,
        conn: &mut AnyConnection,
        roles: &Vec<Role>,
    ) -> result::Result<Self> {
        for role in roles {
            UserMtmRole {
                uid: self.uid,
                rid: role.rid,
            }
            .save(conn)
            .await?;
        }
        Ok(self)
    }
    /// 获取相关roles
    pub async fn get_related_roles(self, conn: &mut AnyConnection) -> result::Result<Vec<Role>> {
        let (query, values) = Query::select()
            // select all fields
            .columns([
                RoleIden::Rid,
                RoleIden::Name,
                RoleIden::Desc,
                RoleIden::IsBase,
                RoleIden::Created,
                RoleIden::Changed,
            ])
            // from user
            .from(UserIden::Table)
            // left join user.uid=user_mtm_role.uid
            .left_join(
                UserMtmRoleIden::Table,
                Expr::col((UserIden::Table, UserIden::Uid))
                    .equals((UserMtmRoleIden::Table, UserMtmRoleIden::Uid)),
            )
            // left join user_mtm_role.rid=role.rid
            .left_join(
                RoleIden::Table,
                Expr::col((UserMtmRoleIden::Table, UserMtmRoleIden::Rid))
                    .equals((RoleIden::Table, RoleIden::Rid)),
            )
            // where user.uid=${uid}
            .and_where(Expr::col((UserIden::Table, UserIden::Uid)).eq(self.uid))
            .build_any_sqlx(&*get_query_builder(conn));
        let roles = sqlx::query_as_with::<_, Role, _>(&query, values)
            .fetch_all(conn)
            .await?;
        Ok(roles)
    }
    /// 获取相关api
    pub async fn get_related_apis(self, conn: &mut AnyConnection) -> result::Result<Vec<Api>> {
        let (query, values) = Query::select()
            // select all fields
            .columns([
                ApiIden::Aid,
                ApiIden::Name,
                ApiIden::Path,
                ApiIden::Method,
                ApiIden::Code,
                ApiIden::Created,
                ApiIden::Changed,
            ])
            // from user
            .from(UserIden::Table)
            // left join user.uid=user_mtm_role.uid
            .left_join(
                UserMtmRoleIden::Table,
                Expr::col((UserIden::Table, UserIden::Uid))
                    .equals((UserMtmRoleIden::Table, UserMtmRoleIden::Uid)),
            )
            // left join user_mtm_role.rid=role.rid
            .left_join(
                RoleIden::Table,
                Expr::col((UserMtmRoleIden::Table, UserMtmRoleIden::Rid))
                    .equals((RoleIden::Table, RoleIden::Rid)),
            )
            // left join role.rid=role_mtm_api.rid
            .left_join(
                RoleMtmAPiIden::Table,
                Expr::col((RoleIden::Table, RoleIden::Rid))
                    .equals((RoleMtmAPiIden::Table, RoleMtmAPiIden::Rid)),
            )
            // left join role_mtm_api.aid=api.aid
            .left_join(
                ApiIden::Table,
                Expr::col((RoleMtmAPiIden::Table, RoleMtmAPiIden::Aid))
                    .equals((ApiIden::Table, ApiIden::Aid)),
            )
            // where user.uid=${uid}
            .and_where(Expr::col((UserIden::Table, UserIden::Uid)).eq(self.uid))
            .build_any_sqlx(&*get_query_builder(conn));
        let apis = sqlx::query_as_with::<_, Api, _>(&query, values)
            .fetch_all(conn)
            .await?;
        Ok(apis)
    }
}
