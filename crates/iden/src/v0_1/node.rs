use sea_query::Iden;

/// node类型Iden
#[derive(Iden)]
pub enum NodeType {
    /// 类型：UUID
    /// ID
    Ntid,
    /// 类型：VARCHAR(64)
    /// node类型名
    Name,
    /// 类型：TEXT
    /// schema
    Schema
    
}

/// 节点Iden
#[derive(Iden)]
pub enum PgNode{
    Table,
    /// 类型:UUID
    /// 节点id
    Nid,
    /// 类型：UUID
    /// 节点版本id
    Vid,
    /// 类型:VARCHAR(32)
    /// 节点类型
    Ntype,
    /// 类型：VARCHAR(8)
    /// 节点默认语言
    Language,
    /// 类型：VARCHAR(32)
    /// 节点标题
    Title,
    /// 类型:VARCHAR(32)
    /// 置顶 NODE_NOT_STICKY|NODE_STICKY
    Sticky,
    /// 类型：UUID
    ///  关联用户id
    Uid,
    /// 类型：VARCHAR(32)
    ///  节点状态 NODE_NOT_PUBLISHED |NODE_PUBLISHED
    Status,
    /// 类型：DATETIME
    ///  节点创建时间
    Created,
    /// 类型：DATETIME
    ///  节点更新时间
    Changed,
    /// 类型：VARCHAR(32)
    ///  评论状态 COMMENT_NODE_HIDDEN | COMMENT_NODE_CLOSED | COMMENT_NODE_OPEN
    Comment,
    /// 类型：UUID
    ///  翻译源节点
    Tnid,
    /// 类型:BOOLEAN
    ///  是否是翻译节点
    Translate,
    /// 类型:json
    ///  节点数据,json格式扩展字段
    Data,
    /// 类型：
    /// 索引
    VectorIdx
}

/// 节点Iden
#[derive(Iden)]
pub enum SqliteNode{
    #[iden(rename="node")]
    Table,
    /// 类型:UUID
    /// 节点id
    Nid,
    /// 类型：UUID
    /// 节点版本id
    Vid,
    /// 类型:UUID
    /// 节点类型
    Ntid,
    /// 类型：VARCHAR(8)
    /// 节点默认语言
    Language,
    /// 类型：VARCHAR(256)
    /// 节点标题
    Title,
    /// 类型:VARCHAR(32)
    /// 置顶 NODE_NOT_STICKY|NODE_STICKY
    Sticky,
    /// 类型：UUID
    ///  关联用户id
    Uid,
    /// 类型：VARCHAR(32)
    ///  节点状态 NODE_NOT_PUBLISHED |NODE_PUBLISHED
    Status,
    /// 类型：DATETIME
    ///  节点创建时间
    Created,
    /// 类型：DATETIME
    ///  节点更新时间
    Changed,
    /// 类型：VARCHAR(32)
    ///  评论状态 COMMENT_NODE_HIDDEN | COMMENT_NODE_CLOSED | COMMENT_NODE_OPEN
    Comment,
    /// 类型：UUID
    ///  翻译源节点
    Tnid,
    /// 类型:json
    ///  节点数据,json格式扩展字段
    Data,
    /// 类型:UUID
    /// doc id,索引
    DocId
}

/// 节点Iden
#[derive(Iden)]
pub enum MysqlNode{
    Table,
    /// 类型:UUID
    /// 节点id
    Nid,
    /// 类型：UUID
    /// 节点版本id
    Vid,
    /// 类型:VARCHAR(32)
    /// 节点类型
    Ntype,
    /// 类型：VARCHAR(8)
    /// 节点默认语言
    Language,
    /// 类型：VARCHAR(32)
    /// 节点标题
    Title,
    /// 类型:VARCHAR(32)
    /// 置顶 NODE_NOT_STICKY|NODE_STICKY
    Sticky,
    /// 类型：UUID
    ///  关联用户id
    Uid,
    /// 类型：VARCHAR(32)
    ///  节点状态 NODE_NOT_PUBLISHED |NODE_PUBLISHED
    Status,
    /// 类型：DATETIME
    ///  节点创建时间
    Created,
    /// 类型：DATETIME
    ///  节点更新时间
    Changed,
    /// 类型：VARCHAR(32)
    ///  评论状态 COMMENT_NODE_HIDDEN | COMMENT_NODE_CLOSED | COMMENT_NODE_OPEN
    Comment,
    /// 类型：UUID
    ///  翻译源节点
    Tnid,
    /// 类型:BOOLEAN
    ///  是否是翻译节点
    Translate,
    /// 类型:json
    ///  节点数据,json格式扩展字段
    Data,
}

/// node版本副本Iden
#[derive(Iden)]
pub enum NodeRevision{
    Table,
    /// 类型：UUID
    /// 版本id,主键
    Vid,
    /// 类型：UUID
    /// 源节点id
    Nid,
    /// 类型：UUID
    /// 修订此版本的用户
    Uid,
    /// 类型：VARCHAR(256)
    /// 修订版本的标题
    Title,
    /// 类型：json
    /// 修订版本的数据
    Data,
    /// 类型：VARCHAR(512)
    /// 修订日志
    Log,
    /// 类型：DATETIME
    /// 修订本的时间戳
    Timestamp
}

