use sea_query::Iden;

///分类学Iden
#[derive(Iden)]
pub enum Taxonomy{
    Table,
    /// 类型：UUID
    /// 分类学ID
    Tid,
    /// 类型：VARCHAR(16)
    /// 名字
    Name,
    /// 类型:VARCHAR(512)
    /// 用于表示层级的path
    Path,
    /// 类型:VARCHAR(2048)
    /// 封面url
    Cover,
    /// 类型:VARCHAR(2048)
    /// 分类学简介
    Desc,
    /// 类型：DATETIME
    /// 创建时间
    Created,
    /// 类型：DATETIME
    /// 更新时间
    Changed,
}
