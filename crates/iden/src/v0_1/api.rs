use sea_query::Iden;

#[derive(Iden)]
pub enum Api {
    Table,
    /// 类型：UUID
    /// ID
    Aid,
    /// 类型：VARCHAR(64)
    /// api名
    Name,
    /// 类型:VARCHAR(2048)
    /// api路由
    Path,
    /// 类型:VARCHAR(8)
    /// api方法，GET,HEAD,POST,PUT,DELETE,CONNECT,OPTIONS,TRACE,PATCH,
    Method,
    /// 类型VARCHAR(256)
    /// 权限编码，供前端
    Code,
    /// 类型：DATETIME
    /// 创建时间
    Created,
    /// 类型：DATETIME
    /// 更新时间
    Changed,
}
