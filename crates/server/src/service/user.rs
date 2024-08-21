use crate::context::ServerContext;
use crate::middleware::jwt::Claims;
use crate::repo;
use models::r#do::user::User;
use models::r#do::utils::ToResult;
use models::req::user::{LoginRequest, RegisterRequest};
use models::resp::user::{LoginResponse, RegisterResponse};
use std::time::Duration;

pub async fn register(
    context: ServerContext,
    req: RegisterRequest,
) -> result::Result<RegisterResponse> {
    let pwd = util::pwd::hash(&req.password).await?;
    let tx = &mut context.db.begin().await?;
    let uid = repo::user::register(tx, &req.username, &req.email, &pwd).await?;
    let resp = RegisterResponse::new(uid);
    // let resp=RegisterResponse::new(Uuid::now_v7());
    Ok(resp)
}

pub async fn login(context: ServerContext, req: LoginRequest) -> result::Result<LoginResponse> {
    let conn = &mut context.db.acquire().await?;
    let user = User::get_by_username(conn, &req.username)
        .await?
        .to_result()?;
    util::pwd::verify(&req.password, &user.password).await?;
    let token = Claims::new(user.uid, Duration::from_secs(context.config.jwt.exp))
        .encode(&context.config.jwt.encoding_key())?;
    let resp = LoginResponse::new(token);
    Ok(resp)
}
