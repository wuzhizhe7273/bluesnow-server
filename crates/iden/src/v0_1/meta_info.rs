use sea_query::Iden;

/// 程序和数据库元信息
#[derive(Iden)]
pub enum MetaInfo{
    Table,
    /// 类型:VARCHAR(16),
    /// 版本
    Version,
    /// 类型:VARCHAR(16),MYSQL,POSTGRES,SQLITE
    /// 数据库
    Database,
    /// 类型:BOOLEAN
    /// 是否初始化
    Initialized,
}