use sea_query::{DeleteStatement, InsertStatement, MysqlQueryBuilder, PostgresQueryBuilder, QueryStatementWriter, SelectStatement, SqliteQueryBuilder, UpdateStatement};
use sqlx::{Acquire, Any, AnyConnection, Database, Sqlite, SqliteConnection};
use sqlx::query::Query;

pub type NowDatabase=Sqlite;
pub type NowConnection=SqliteConnection;
pub trait ToSqlxQuery{
    async fn to_sqlx_query(&self,conn:&NowConnection)->anyhow::Result<String>;
}

impl ToSqlxQuery for &mut InsertStatement{
    async fn to_sqlx_query(&self, conn:&NowConnection) -> anyhow::Result<String>
    {
        let backend_name=conn.backend_name();
        let query=match backend_name{
            "PostgreSql"=> self.to_string(PostgresQueryBuilder),
            "MySql"=>self.to_string(MysqlQueryBuilder),
            "SQLite"=>self.to_string(SqliteQueryBuilder),
            _=>
                panic!("不受支持的数据库")
        };
        Ok(query)
    }
}

impl ToSqlxQuery for &mut SelectStatement{
    async fn to_sqlx_query(&self, conn:&NowConnection) -> anyhow::Result<String>
    {
        let backend_name=conn.backend_name();
        let query=match backend_name{
            "PostgreSql"=> self.to_string(PostgresQueryBuilder),
            "MySql"=>self.to_string(MysqlQueryBuilder),
            "SQLite"=>self.to_string(SqliteQueryBuilder),
            _=>
                panic!("不受支持的数据库")
        };
        Ok(query)
    }
}

impl ToSqlxQuery for &mut DeleteStatement{
    async fn to_sqlx_query(&self, conn:&NowConnection) -> anyhow::Result<String>
    {
        let backend_name=conn.backend_name();
        let query=match backend_name{
            "PostgreSql"=> self.to_string(PostgresQueryBuilder),
            "MySql"=>self.to_string(MysqlQueryBuilder),
            "SQLite"=>self.to_string(SqliteQueryBuilder),
            _=>
                panic!("不受支持的数据库")
        };
        Ok(query)
    }
}

impl ToSqlxQuery for &mut UpdateStatement{
    async fn to_sqlx_query(&self, conn:&NowConnection) -> anyhow::Result<String>
    {
        let backend_name=conn.backend_name();
        let query=match backend_name{
            "PostgreSql"=> self.to_string(PostgresQueryBuilder),
            "MySql"=>self.to_string(MysqlQueryBuilder),
            "SQLite"=>self.to_string(SqliteQueryBuilder),
            _=>
                panic!("不受支持的数据库")
        };
        Ok(query)
    }
}