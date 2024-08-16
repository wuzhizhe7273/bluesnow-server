pub struct DbConfig{
    // pub db_type:DbType,
    pub url:String
}

pub enum DbType {
    Mysql,
    Sqlte,
    Postgres
}