pub use config::ServerConfig;
use context::ServerContext;
use tokio::signal;
use crate::routes::Routes;

mod service;
mod repo;
mod handler;
mod middleware;
mod config;
mod context;

#[cfg(feature = "log")]
mod log;
mod routes;

pub struct Server{
    config:ServerConfig
}
impl Server {
    pub fn new(config:ServerConfig)->Self{
        Self{config}
    }
    pub async fn run(&self)->anyhow::Result<()>{
        let listener=tokio::net::TcpListener::bind(("127.0.0.1",self.config.port)).await?;
        let context=ServerContext::new(&self.config).await?;
        let router=Routes::new(context).build();
        #[cfg(feature = "log")]
        let router=log::Log::new(&self.config.log).init().set_router(router);
        #[cfg(feature = "log")]
        {
            tracing::info!("server is running, listening on 127.0.0.1:{}",self.config.port);
            tracing::debug!("you can access api by http://127.0.0.1:{}",self.config.port);
        }

        axum::serve(listener, router.into_make_service()).with_graceful_shutdown(showdown_signal()).await?;
        Ok(())
    }
}

async fn showdown_signal(){
    let ctrl_c=async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler")
    };
    #[cfg(unix)]
    let terminate=async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate=std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c =>{},
        _ = terminate=>{}
    }
}