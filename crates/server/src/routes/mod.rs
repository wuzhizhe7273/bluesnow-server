use axum::Router;
use axum::routing::{get, post};
use crate::context::ServerContext;
use crate::handler;

pub struct Routes{
    context:ServerContext
}

impl Routes{
    pub fn new(context:ServerContext)->Self{
        Routes{
            context
        }
    }
    pub fn build(self)->Router{
        Router::new()
            .route("/ping",get(||async { "server is running" }))
            .route("/user/register",post(handler::user::register))
            .with_state(self.context)
    }
}