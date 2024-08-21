use crate::config::jwt::JwtConfig;
#[cfg(feature = "log")]
use crate::log::LogConfig;
use cfg_if::cfg_if;
use db::DbConfig;

mod db;
mod jwt;
#[derive(Debug,Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub db: DbConfig,
    #[cfg(feature = "log")]
    pub log: LogConfig,
    pub jwt: JwtConfig,
}

impl ServerConfig {
    pub fn new() -> Self {
        let port = 3000;
        let db = DbConfig {
            url: "sqlite://data/db.sqlite?mode=rwc".to_string(),
        };
        #[cfg(feature = "log")]
        let log = LogConfig {
            level: "debug".to_string(),
        };
        let jwt = JwtConfig {
            secret: "test secret".to_string(),
            exp:60*60*12
        };
        cfg_if! {
            if #[cfg(feature="log")]{
                Self{
                    port,
                    db,
                    log,
                   jwt
                }
            }else{
                Self{
                    port,
                    jwt,
                    db
                }
            }
        }
    }
}
