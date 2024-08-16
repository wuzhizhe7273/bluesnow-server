use sea_query::Iden;

/// user和role的多对多关系表
#[derive(Debug,Iden)]
pub enum UserMtmRole {
    Table,
    /// 类型：UUID
    /// user ID
    Uid,
    /// 类型:UUID
    /// role ID 
    Rid,
}
