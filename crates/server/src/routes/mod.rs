use crate::context::ServerContext;
use crate::handler;
use crate::middleware::jwt::mid_access_control;
use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::Router;

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
            .route("/user/login",post(handler::user::login))
            .layer(from_fn_with_state(self.context.clone(),mid_access_control))
            .with_state(self.context)
    }
}