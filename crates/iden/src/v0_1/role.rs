use sea_query::Iden;
/// 角色表Iden
#[derive(Iden)]
pub enum Role {
    Table,
    /// 类型:UUID
    /// 角色ID
    Rid, 
    /// 类型:VARCHAR(64)
    /// 角色名
    Name,
    /// 类型:VARCHAR(2048)
    /// 角色简介
    Desc,
    /// 类型:BOOLEAN
    /// 是否为基础角色
    IsBase,
    /// 类型:DATETIME
    /// 创建时间
    Created,
    /// 类型:DATETIME
    /// 更新时间
    Updated,
}
