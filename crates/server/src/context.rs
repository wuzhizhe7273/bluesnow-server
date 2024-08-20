use crate::ServerConfig;
use sqlx::AnyPool;
use std::sync::Arc;
#[derive(Debug,Clone)]
pub struct ServerContext{
    pub db:AnyPool,
    pub config:Arc<ServerConfig>
}

impl ServerContext {
    pub async fn new(config:&ServerConfig)->anyhow::Result<Self>{
        sqlx::any::install_default_drivers();
        let pool=AnyPool::connect(&config.db.url).await?;
        Ok(Self{
            db:pool,
            config:Arc::new(config.clone())
        })
    }
}