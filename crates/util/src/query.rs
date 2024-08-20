use sea_query::{MysqlQueryBuilder, PostgresQueryBuilder, QueryBuilder, SqliteQueryBuilder};
use sqlx::AnyConnection;

// #[macro_export]
// macro_rules! sea_query_statement_to_string {
//     ($query:expr;$conn:expr) => {
//         match $conn.backend_name() {
//             "SQLite"=>$query.to_string(sea_query::SqliteQueryBuilder),
//             "MySQL"=>$query.to_string(sea_query::MysqlQueryBuilder),
//             "PostgreSQL"=>$query.to_string(sea_query::PostgresQueryBuilder),
//             _=>panic!("unsupported database backend")
//         }
//     };
// }

pub fn get_query_builder(conn:&mut AnyConnection)->Box<dyn QueryBuilder>{
    match conn.backend_name() {
        "PostgreSQL"=>Box::new(PostgresQueryBuilder),
        "MySQL"=>Box::new(MysqlQueryBuilder),
        "SQLite"=>Box::new(SqliteQueryBuilder),
        _=>{panic!("Unsupported Database")}
    }
}