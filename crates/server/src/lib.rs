use axum::{routing::get, ServiceExt};
pub use config::ServerConfig;
use context::ServerContext;

mod service;
mod repo;
mod handler;
mod middleware;
mod config;
mod context;

pub struct Server{
    config:ServerConfig
}
impl Server {
    pub fn new(config:ServerConfig)->Self{
        Self{config}
    }
    pub async fn run(&self)->anyhow::Result<()>{
        let listener=tokio::net::TcpListener::bind(("127.0.0.1",self.config.port)).await?;
        let router=axum::Router::new().route("/ping", get(||async{"Server is running"}));
        let context=ServerContext::new(&self.config).await?;
        let router=router.with_state(context);
        axum::serve(listener, router.into_make_service()).await?;
        Ok(())
    }
}