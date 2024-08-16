use sea_query::Iden;

/// node 和 taxonomy的多对多关系表Iden
#[derive(Iden)]
pub enum NodeMtmTaxonomy{
    Table,
    /// 类型：UUID
    /// node ID
    Nid,
    /// 类型：UUID
    /// taxonomy ID
    Tid
}