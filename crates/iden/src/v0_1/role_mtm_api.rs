use sea_query::Iden;

/// role 和 api 的多对多关系表Iden
#[derive(Iden)]
pub enum RoleMtmApi {
    Table,
    /// 类型:UUID
    /// role ID
    Rid,
    /// 类型：UUID
    /// api ID
    Aid,
}
