#[macro_export]
macro_rules! sea_query_statement_to_string {
    ($query:expr;$conn:expr) => {
        match $conn.backend_name() {
            "SQLite"=>$query.to_string(sea_query::SqliteQueryBuilder),
            "MySQL"=>$query.to_string(sea_query::MysqlQueryBuilder),
            "PostgreSQL"=>$query.to_string(sea_query::PostgresQueryBuilder),
            _=>panic!("unsupported database backend")
        }
    };
}