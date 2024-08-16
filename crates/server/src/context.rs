use crate::ServerConfig;

#[derive(Debug,Clone)]
pub struct ServerContext{
    db:sqlx::AnyPool
}

impl ServerContext {
    pub async fn new(config:&ServerConfig)->anyhow::Result<Self>{
        sqlx::any::install_default_drivers();
        let pool=sqlx::AnyPool::connect(&config.db.url).await?;
        Ok(Self{
            db:pool
        })
    }
}