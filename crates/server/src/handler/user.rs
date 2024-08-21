use crate::context::ServerContext;
use crate::service;
use axum::extract::Json;
use axum::extract::State;
use garde::Validate;
use models::req::user::{LoginRequest, RegisterRequest};
use models::resp::user::{LoginResponse, RegisterResponse};

pub async fn register(
    State(context):State<ServerContext>,
    Json(req):Json<RegisterRequest>
)->result::Result<Json<RegisterResponse>>{
    req.validate()?;
    let resp=service::user::register(context,req).await?;
    Ok(Json(resp))
}

pub async fn login(State(context):State<ServerContext>,Json(req):Json<LoginRequest>)->result::Result<Json<LoginResponse>>{
    req.validate()?;
    let resp=service::user::login(context,req).await?;
    Ok(Json(resp))
}