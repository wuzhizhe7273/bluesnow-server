use sea_query::Iden;

/// 用户表Iden
#[derive(Copy,Clone,Iden)]
pub enum User {
    Table,
    /// 类型：UUID
    /// 用户Id
    Uid,
    /// 类型:VARCHAR(64)
    /// 用户名
    Username,
    /// 类型:VARCHAR(256)
    /// 用户密码，argon2加密，phc字符串
    Password,
    /// 类型: VARCHAR(256)
    /// 用户邮箱
    Email,
    /// 类型: UUID
    /// 激活的用户角色
    ActiveRid,
    /// 类型:DATETIME
    /// 创建时间
    Created,
    /// 类型：DATETIME
    /// 更新时间
    Changed,
}
