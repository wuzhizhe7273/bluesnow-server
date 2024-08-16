use sea_query::Iden;
/// role 和 menu的多对多关系表
#[derive(Debug,Iden)]
pub enum RoleMtmMenu {
    Table,
    /// 类型：UUID
    /// role ID
    Rid,
    /// 类型：UUID
    /// menu ID
    Mid,
}
