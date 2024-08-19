use sqlx::AnyPool;
use crate::ServerConfig;
#[derive(Debug,Clone)]
pub struct ServerContext{
    pub db:AnyPool
}

impl ServerContext {
    pub async fn new(config:&ServerConfig)->anyhow::Result<Self>{
        sqlx::any::install_default_drivers();
        let pool=AnyPool::connect(&config.db.url).await?;
        Ok(Self{
            db:pool
        })
    }
}