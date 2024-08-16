use sea_query::Iden;


/// menu表Iden
#[derive(Debug,Iden)]
pub enum Menu {
    Table,
    /// 类型：UUID
    /// 菜单ID
    Mid,
    /// 类型:VARCHAR(2048)
    /// 指向的url
    Href,
    /// 类型:VARCHAR(128)
    /// 菜单名
    Name,
    /// 类型:VARCHAR(2048)
    /// 组件路径
    Component,
    /// 类型:BOOLEAN
    /// 显示状态
    IsVisible,
    /// 类型：VARCHAR(64)
    /// 状态
    Status,
    /// 类型：BOOLEAN
    /// 前端KeepAlive 
    KeepAlive,
    /// 类型：INT
    /// 排序
    Order,
    /// 类型：UUID
    /// 父菜单Id
    Pid,
    /// 类型：DATETIME
    /// 创建时间
    Created,
    /// 类型：DATETIME
    /// 更新时间
    Changed
}
