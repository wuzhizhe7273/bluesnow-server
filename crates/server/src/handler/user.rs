use axum::extract::State;
use axum::extract::Json;
use garde::Validate;
use models::req::user::RegisterRequest;
use models::resp::user::RegisterResponse;
use crate::context::ServerContext;
use crate::service;

pub async fn register(
    State(context):State<ServerContext>,
    Json(req):Json<RegisterRequest>
)->result::Result<Json<RegisterResponse>>{
    req.validate()?;
    let resp=service::user::register(context,req).await?;
    Ok(Json(resp))
}