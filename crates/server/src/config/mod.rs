use db::DbConfig;

mod db;
pub struct ServerConfig{
    pub port:u16,
    pub db:DbConfig
}

impl ServerConfig {
    pub fn new()->Self{
        Self{port:3001,db:DbConfig{url:"sqlite://data/db.sqlite?mode=rwc".to_string()}}
    }
}